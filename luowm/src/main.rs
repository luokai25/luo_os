mod types;
mod framebuffer;
mod window;
mod taskbar;
mod desktop;
mod input;
mod ipc;
mod agent;
mod compositor;

use compositor::Compositor;
use input::{InputEvent, MouseButton, Key};
use types::Point;
use std::time::{Duration, Instant};
use chrono::Local;

fn main() {
    println!("luowm v0.1 — luo_os window manager");
    println!("Initializing compositor...");

    let mut comp = Compositor::new();

    println!("Opening default windows...");

    // open welcome window on startup
    let welcome_id = comp.open_window("Welcome to luo_os", 120, 60, 500, 300);
    comp.append_to_window(welcome_id, "luo_os v1.0 Desktop");
    comp.append_to_window(welcome_id, "");
    comp.append_to_window(welcome_id, "Click desktop icons to open apps:");
    comp.append_to_window(welcome_id, "  >_  Terminal   — run shell commands");
    comp.append_to_window(welcome_id, "  []  Files      — browse filesystem");
    comp.append_to_window(welcome_id, "  ()  AI Agent   — talk to the OS");
    comp.append_to_window(welcome_id, "   i  About      — about luo_os");
    comp.append_to_window(welcome_id, "");
    comp.append_to_window(welcome_id, "AI agents can control this desktop via:");
    comp.append_to_window(welcome_id, "  agent/daemon.py");

    println!("Starting event loop...");
    println!("Framebuffer: {}x{}", framebuffer::SCREEN_W, framebuffer::SCREEN_H);
    println!("luowm running. Simulating 5 frames then saving state.");

    // ── demo event loop ────────────────────────────────────
    // In production this connects to /dev/fb0 or a display server
    // Here we simulate events and prove the compositor works

    let start = Instant::now();
    let mut frame = 0u64;
    let mut last_clock = Instant::now();

    // simulate some input events to prove the system works
    let events = vec![
        InputEvent::MouseMove(Point { x: 200, y: 200 }),
        InputEvent::MouseDown(Point { x: 20, y: 40 }, MouseButton::Left),  // click terminal icon
        InputEvent::MouseUp(Point { x: 20, y: 40 }, MouseButton::Left),
        InputEvent::MouseMove(Point { x: 400, y: 300 }),
        InputEvent::MouseDown(Point { x: 150, y: 65 }, MouseButton::Left), // drag welcome window
        InputEvent::MouseMove(Point { x: 160, y: 75 }),
        InputEvent::MouseMove(Point { x: 180, y: 90 }),
        InputEvent::MouseUp(Point { x: 180, y: 90 }, MouseButton::Left),
        InputEvent::KeyDown(Key::Char('h')),
        InputEvent::KeyUp(Key::Char('h')),
    ];

    for ev in events {
        comp.handle_input(ev);
    }

    // simulate agent command
    comp.agents_handle().push(agent::AgentCommand::OpenWindow {
        title: "AI Output".to_string(),
        x: 350, y: 150, w: 400, h: 250,
    });
    comp.agents_handle().push(agent::AgentCommand::AppendLine {
        id: comp.next_id - 1,
        line: "Agent connected. Awaiting instructions.".to_string(),
    });

    // run 10 render frames
    loop {
        frame += 1;

        // update clock
        if last_clock.elapsed() >= Duration::from_millis(500) {
            let now = Local::now();
            comp.set_clock(&now.format("%H:%M").to_string());
            last_clock = Instant::now();
        }

        comp.process_agent_commands();
        comp.render();

        println!("[frame {}] rendered {}x{} px, {} windows open",
            frame,
            comp.fb.width,
            comp.fb.height,
            comp.windows.len(),
        );

        if frame >= 10 { break; }
        std::thread::sleep(Duration::from_millis(16)); // ~60fps
    }

    let elapsed = start.elapsed();
    println!("");
    println!("=== luowm session summary ===");
    println!("Frames rendered  : {}", frame);
    println!("Total time       : {:.2}s", elapsed.as_secs_f32());
    println!("Avg frame time   : {:.1}ms", elapsed.as_millis() as f32 / frame as f32);
    println!("Windows opened   : {}", comp.next_id - 1);
    println!("Framebuffer size : {} bytes", comp.fb.buf.len());
    println!("");
    println!("luowm: compositor working. Ready for display output.");
    println!("Next: connect to /dev/fb0 for real screen output.");
}
