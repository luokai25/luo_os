use crate::types::{Rect, Color, Point};
use crate::framebuffer::Framebuffer;

#[derive(Debug, Clone, PartialEq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Closing,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub id:       u32,
    pub title:    String,
    pub rect:     Rect,
    pub state:    WindowState,
    pub focused:  bool,
    pub content:  Vec<String>,   // text lines displayed in window body
    pub dirty:    bool,
}

impl Window {
    pub fn new(id: u32, title: &str, x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            id,
            title:   title.to_string(),
            rect:    Rect::new(x, y, w, h),
            state:   WindowState::Normal,
            focused: false,
            content: Vec::new(),
            dirty:   true,
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.content.push(line.to_string());
        self.dirty = true;
    }

    pub fn clear_content(&mut self) {
        self.content.clear();
        self.dirty = true;
    }

    pub fn set_content(&mut self, lines: Vec<String>) {
        self.content = lines;
        self.dirty = true;
    }

    pub fn draw(&mut self, fb: &mut Framebuffer) {
        if self.state == WindowState::Minimized { return; }

        let r = self.rect;

        // window shadow
        fb.fill_rect(Rect::new(r.x+4, r.y+4, r.w, r.h), Color::rgb(0,0,0));

        // window background
        fb.fill_rect(r, Color::rgb(25, 25, 35));

        // titlebar
        let tb_color = if self.focused {
            Color::TITLEBAR_FOCUS
        } else {
            Color::TITLEBAR
        };
        let tb = r.titlebar();
        fb.fill_rect(tb, tb_color);

        // title text
        let title_short: String = self.title.chars().take(30).collect();
        fb.draw_text(&title_short, tb.x + 8, tb.y + 8, Color::WHITE);

        // close button
        let cb = r.close_btn();
        fb.fill_rect(cb, Color::RED);
        fb.draw_text("x", cb.x + 4, cb.y + 4, Color::WHITE);

        // minimize button
        let mb = r.min_btn();
        fb.fill_rect(mb, Color::AMBER);
        fb.draw_text("-", mb.x + 5, mb.y + 4, Color::WHITE);

        // window border
        fb.draw_rect_outline(r, if self.focused {
            Color::ACCENT
        } else {
            Color::GRAY
        }, 1);

        // content area
        let content_y = tb.y + tb.h as i32 + 4;
        let content_x = r.x + 8;
        let max_lines = ((r.h as i32 - tb.h as i32 - 12) / 10) as usize;
        let start = if self.content.len() > max_lines {
            self.content.len() - max_lines
        } else {
            0
        };
        for (i, line) in self.content[start..].iter().enumerate() {
            let short: String = line.chars().take(((r.w - 16) / 6) as usize).collect();
            fb.draw_text(&short, content_x, content_y + i as i32 * 10, Color::LIGHT_GRAY);
        }

        self.dirty = false;
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
        self.dirty  = true;
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.rect.w = w.max(120);
        self.rect.h = h.max(80);
        self.dirty  = true;
    }

    pub fn hit_close(&self, p: Point) -> bool {
        self.rect.close_btn().contains(p)
    }

    pub fn hit_minimize(&self, p: Point) -> bool {
        self.rect.min_btn().contains(p)
    }

    pub fn hit_titlebar(&self, p: Point) -> bool {
        self.rect.titlebar().contains(p)
            && !self.hit_close(p)
            && !self.hit_minimize(p)
    }

    pub fn hit_body(&self, p: Point) -> bool {
        self.rect.contains(p) && !self.rect.titlebar().contains(p)
    }
}
