use crate::framebuffer::Framebuffer;
use crate::window::Window;
use crate::taskbar::Taskbar;
use crate::desktop::Desktop;
use crate::input::{InputEvent, InputState, MouseButton, draw_cursor};
use crate::types::Point;
use crate::agent::{AgentCommand, AgentQueue};
use crate::ipc::KernelIpc;
use std::sync::Arc;

pub struct Compositor {
    pub fb:       Framebuffer,
    pub windows:  Vec<Window>,
    pub taskbar:  Taskbar,
    pub desktop:  Desktop,
    pub input:    InputState,
    pub next_id:  u32,
    pub agents:   Arc<AgentQueue>,
    pub ipc:      KernelIpc,
    drag_win_id:  Option<u32>,
    drag_offset:  (i32, i32),
}

impl Compositor {
    pub fn new() -> Self {
        let agents = Arc::new(AgentQueue::new());
        let mut ipc = KernelIpc::new("/dev/ttyS0");
        ipc.connect();

        Self {
            fb:          Framebuffer::new(),
            windows:     Vec::new(),
            taskbar:     Taskbar::new(),
            desktop:     Desktop::new(),
            input:       InputState::new(),
            next_id:     1,
            agents,
            ipc,
            drag_win_id: None,
            drag_offset: (0, 0),
        }
    }

    pub fn open_window(&mut self, title: &str, x: i32, y: i32, w: u32, h: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let mut win = Window::new(id, title, x, y, w, h);
        win.focused = true;
        // unfocus all others
        for w in &mut self.windows { w.focused = false; }
        self.taskbar.add_button(id, title);
        self.taskbar.set_active(id);
        self.windows.push(win);
        id
    }

    pub fn close_window(&mut self, id: u32) {
        self.windows.retain(|w| w.id != id);
        self.taskbar.remove_button(id);
        // focus last window
        if let Some(last) = self.windows.last_mut() {
            last.focused = true;
            let lid = last.id;
            self.taskbar.set_active(lid);
        }
    }

    pub fn focus_window(&mut self, id: u32) {
        for w in &mut self.windows { w.focused = w.id == id; }
        self.taskbar.set_active(id);
        // bring to front
        if let Some(pos) = self.windows.iter().position(|w| w.id == id) {
            let win = self.windows.remove(pos);
            self.windows.push(win);
        }
    }

    pub fn write_to_window(&mut self, id: u32, lines: Vec<String>) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.set_content(lines);
        }
    }

    pub fn append_to_window(&mut self, id: u32, line: &str) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.add_line(line);
        }
    }

    /// Process one input event — hit testing, drag, focus
    pub fn handle_input(&mut self, ev: InputEvent) {
        self.input.process(&ev);
        let p = self.input.mouse_pos;

        match &ev {
            InputEvent::MouseDown(click, MouseButton::Left) => {
                let cp = *click;

                // check windows in reverse (top first)
                let mut action: Option<&str> = None;
                let mut act_id: u32 = 0;

                for win in self.windows.iter().rev() {
                    if win.hit_close(cp) {
                        action = Some("close"); act_id = win.id; break;
                    }
                    if win.hit_minimize(cp) {
                        action = Some("minimize"); act_id = win.id; break;
                    }
                    if win.hit_titlebar(cp) {
                        action = Some("drag"); act_id = win.id; break;
                    }
                    if win.hit_body(cp) {
                        action = Some("focus"); act_id = win.id; break;
                    }
                }

                match action {
                    Some("close")    => { self.close_window(act_id); }
                    Some("minimize") => {
                        if let Some(w) = self.windows.iter_mut().find(|w| w.id == act_id) {
                            w.state = crate::window::WindowState::Minimized;
                        }
                    }
                    Some("drag") => {
                        self.focus_window(act_id);
                        if let Some(w) = self.windows.iter().find(|w| w.id == act_id) {
                            self.drag_win_id = Some(act_id);
                            self.drag_offset = (cp.x - w.rect.x, cp.y - w.rect.y);
                        }
                    }
                    Some("focus") => { self.focus_window(act_id); }
                    _ => {
                        // click on taskbar
                        if let Some(win_id) = self.taskbar.hit_button(cp) {
                            if let Some(w) = self.windows.iter_mut().find(|w| w.id == win_id) {
                                if w.state == crate::window::WindowState::Minimized {
                                    w.state = crate::window::WindowState::Normal;
                                }
                            }
                            self.focus_window(win_id);
                        }
                        // click on desktop icon
                        else if let Some(action) = self.desktop.hit_icon(cp).cloned() {
                            self.handle_desktop_action(action);
                        }
                    }
                }
            }

            InputEvent::MouseUp(_, MouseButton::Left) => {
                self.drag_win_id = None;
            }

            InputEvent::MouseMove(mp) => {
                if let Some(id) = self.drag_win_id {
                    let new_x = mp.x - self.drag_offset.0;
                    let new_y = mp.y - self.drag_offset.1;
                    if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
                        w.move_to(new_x.max(0), new_y.max(0));
                    }
                }
            }

            InputEvent::KeyDown(key) => {
                // send key to focused window
                if let Some(w) = self.windows.iter().rev().find(|w| w.focused) {
                    let _id = w.id;
                    // future: route to app input handler
                    let _ = key;
                }
            }

            _ => {}
        }

        let _ = p;
    }

    fn handle_desktop_action(&mut self, action: crate::desktop::DesktopAction) {
        use crate::desktop::DesktopAction;
        match action {
            DesktopAction::OpenTerminal => {
                let id = self.open_window("Terminal", 100, 80, 500, 320);
                let output = self.ipc.send_command("version");
                for line in output.lines() {
                    self.append_to_window(id, line);
                }
                self.append_to_window(id, "luo_os:~$ ");
            }
            DesktopAction::OpenFileManager => {
                let id = self.open_window("File Manager", 200, 100, 480, 360);
                let output = self.ipc.send_command("ls");
                self.append_to_window(id, "=== luo_os filesystem ===");
                for line in output.lines() {
                    self.append_to_window(id, line);
                }
            }
            DesktopAction::OpenAIAgent => {
                let id = self.open_window("AI Agent", 300, 120, 520, 380);
                self.append_to_window(id, "=== luo_os AI Agent ===");
                self.append_to_window(id, "Status: daemon.py not connected");
                self.append_to_window(id, "Run: python3 agent/daemon.py --mock");
                self.append_to_window(id, "");
                self.append_to_window(id, "Capabilities:");
                self.append_to_window(id, "  - Natural language OS commands");
                self.append_to_window(id, "  - File read/write");
                self.append_to_window(id, "  - Process management");
                self.append_to_window(id, "  - Window control API");
            }
            DesktopAction::OpenAbout => {
                let id = self.open_window("About luo_os", 250, 200, 400, 280);
                self.append_to_window(id, "luo_os v1.0");
                self.append_to_window(id, "");
                self.append_to_window(id, "Built from scratch by luokai25");
                self.append_to_window(id, "");
                self.append_to_window(id, "Stack:");
                self.append_to_window(id, "  Kernel:  C + ASM (x86-32)");
                self.append_to_window(id, "  Desktop: Rust (luowm)");
                self.append_to_window(id, "  AI:      Python (daemon.py)");
                self.append_to_window(id, "");
                self.append_to_window(id, "Goal: Human + AI desktop OS");
            }
        }
    }

    /// Process all pending agent commands
    pub fn process_agent_commands(&mut self) {
        let cmds = self.agents.drain();
        for cmd in cmds {
            self.handle_agent_command(cmd);
        }
    }

    fn handle_agent_command(&mut self, cmd: AgentCommand) {
        match cmd {
            AgentCommand::OpenWindow { title, x, y, w, h } => {
                self.open_window(&title, x, y, w, h);
            }
            AgentCommand::CloseWindow { id } => {
                self.close_window(id);
            }
            AgentCommand::MoveWindow { id, x, y } => {
                if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
                    win.move_to(x, y);
                }
            }
            AgentCommand::ResizeWindow { id, w, h } => {
                if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
                    win.resize(w, h);
                }
            }
            AgentCommand::WriteContent { id, lines } => {
                self.write_to_window(id, lines);
            }
            AgentCommand::AppendLine { id, line } => {
                self.append_to_window(id, &line);
            }
            AgentCommand::ClearContent { id } => {
                if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
                    win.clear_content();
                }
            }
            AgentCommand::FocusWindow { id } => {
                self.focus_window(id);
            }
            AgentCommand::RunCommand { cmd } => {
                let output = self.ipc.send_command(&cmd);
                // find focused window and write output there
                if let Some(win) = self.windows.iter().rev().find(|w| w.focused) {
                    let id = win.id;
                    for line in output.lines() {
                        self.append_to_window(id, line);
                    }
                }
            }
            AgentCommand::ListWindows => { /* response handled by caller */ }
        }
    }

    /// Render one full frame
    pub fn render(&mut self) {
        // 1. desktop background + icons
        self.desktop.draw(&mut self.fb);

        // 2. all windows bottom to top
        for i in 0..self.windows.len() {
            // temporarily take window out to pass fb
            let mut win = self.windows[i].clone();
            win.draw(&mut self.fb);
            self.windows[i] = win;
        }

        // 3. taskbar on top
        self.taskbar.draw(&mut self.fb);

        // 4. mouse cursor last (always on top)
        draw_cursor(&mut self.fb, self.input.mouse_pos);
    }

    /// Update clock string
    pub fn set_clock(&mut self, time_str: &str) {
        self.taskbar.set_clock(time_str);
    }

    pub fn agents_handle(&self) -> Arc<AgentQueue> {
        Arc::clone(&self.agents)
    }
}
