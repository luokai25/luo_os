use crate::types::{Rect, Color, Point};
use crate::framebuffer::{Framebuffer, SCREEN_W, SCREEN_H};

pub const TASKBAR_H: u32 = 32;

pub struct Taskbar {
    pub rect:     Rect,
    pub buttons:  Vec<TaskButton>,
    pub clock_str: String,
}

#[derive(Debug, Clone)]
pub struct TaskButton {
    pub rect:     Rect,
    pub label:    String,
    pub win_id:   u32,
    pub active:   bool,
}

impl Taskbar {
    pub fn new() -> Self {
        let rect = Rect::new(
            0,
            SCREEN_H as i32 - TASKBAR_H as i32,
            SCREEN_W,
            TASKBAR_H,
        );
        Self {
            rect,
            buttons:   Vec::new(),
            clock_str: String::from("00:00"),
        }
    }

    pub fn add_button(&mut self, win_id: u32, label: &str) {
        let x = 4 + self.buttons.len() as i32 * 104;
        self.buttons.push(TaskButton {
            rect:   Rect::new(self.rect.x + x, self.rect.y + 4, 100, 24),
            label:  label.chars().take(14).collect(),
            win_id,
            active: false,
        });
    }

    pub fn remove_button(&mut self, win_id: u32) {
        self.buttons.retain(|b| b.win_id != win_id);
        // reposition remaining buttons
        for (i, btn) in self.buttons.iter_mut().enumerate() {
            btn.rect.x = 4 + i as i32 * 104;
        }
    }

    pub fn set_active(&mut self, win_id: u32) {
        for btn in &mut self.buttons {
            btn.active = btn.win_id == win_id;
        }
    }

    pub fn set_clock(&mut self, time_str: &str) {
        self.clock_str = time_str.to_string();
    }

    pub fn hit_button(&self, p: Point) -> Option<u32> {
        for btn in &self.buttons {
            if btn.rect.contains(p) {
                return Some(btn.win_id);
            }
        }
        None
    }

    pub fn draw(&self, fb: &mut Framebuffer) {
        // taskbar background
        fb.fill_rect(self.rect, Color::TASKBAR_BG);

        // top border line
        for x in 0..SCREEN_W as i32 {
            fb.set_pixel(x, self.rect.y, Color::ACCENT);
        }

        // start button
        let start_rect = Rect::new(self.rect.x + 4, self.rect.y + 4, 60, 24);
        fb.fill_rect(start_rect, Color::ACCENT);
        fb.draw_text_centered("LUO OS", start_rect, Color::WHITE);

        // separator
        for y in self.rect.y + 4..self.rect.y + 28 {
            fb.set_pixel(68, y, Color::GRAY);
        }

        // window buttons
        for btn in &self.buttons {
            let color = if btn.active {
                Color::ACCENT_DARK
            } else {
                Color::GRAY
            };
            fb.fill_rect(btn.rect, color);
            fb.draw_rect_outline(btn.rect, Color::LIGHT_GRAY, 1);
            fb.draw_text(&btn.label, btn.rect.x + 4, btn.rect.y + 8, Color::WHITE);
        }

        // clock (right side)
        let clock_x = SCREEN_W as i32 - 60;
        let clock_rect = Rect::new(clock_x, self.rect.y + 4, 56, 24);
        fb.fill_rect(clock_rect, Color::rgb(10, 10, 10));
        fb.draw_text_centered(&self.clock_str, clock_rect, Color::WHITE);

        // system tray separator
        for y in self.rect.y + 4..self.rect.y + 28 {
            fb.set_pixel(clock_x - 4, y, Color::GRAY);
        }
    }
}
