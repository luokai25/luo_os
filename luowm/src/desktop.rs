use crate::types::{Rect, Color, Point};
use crate::framebuffer::{Framebuffer, SCREEN_W, SCREEN_H};
use crate::taskbar::TASKBAR_H;

pub struct DesktopIcon {
    pub rect:    Rect,
    pub label:   String,
    pub action:  DesktopAction,
}

#[derive(Debug, Clone)]
pub enum DesktopAction {
    OpenTerminal,
    OpenFileManager,
    OpenAbout,
    OpenAIAgent,
}

pub struct Desktop {
    pub icons: Vec<DesktopIcon>,
}

impl Desktop {
    pub fn new() -> Self {
        let mut icons = Vec::new();

        // Terminal icon
        icons.push(DesktopIcon {
            rect:   Rect::new(20, 40, 64, 64),
            label:  "Terminal".to_string(),
            action: DesktopAction::OpenTerminal,
        });

        // Files icon
        icons.push(DesktopIcon {
            rect:   Rect::new(20, 140, 64, 64),
            label:  "Files".to_string(),
            action: DesktopAction::OpenFileManager,
        });

        // AI Agent icon
        icons.push(DesktopIcon {
            rect:   Rect::new(20, 240, 64, 64),
            label:  "AI Agent".to_string(),
            action: DesktopAction::OpenAIAgent,
        });

        // About icon
        icons.push(DesktopIcon {
            rect:   Rect::new(20, 340, 64, 64),
            label:  "About".to_string(),
            action: DesktopAction::OpenAbout,
        });

        Self { icons }
    }

    pub fn hit_icon(&self, p: Point) -> Option<&DesktopAction> {
        for icon in &self.icons {
            if icon.rect.contains(p) {
                return Some(&icon.action);
            }
        }
        None
    }

    pub fn draw(&self, fb: &mut Framebuffer) {
        // draw background gradient (dark blue-black)
        for y in 0..(SCREEN_H - TASKBAR_H) as i32 {
            let shade = (y as f32 / (SCREEN_H - TASKBAR_H) as f32 * 20.0) as u8;
            let c = Color::rgb(5 + shade/2, 5 + shade/2, 15 + shade);
            for x in 0..SCREEN_W as i32 {
                fb.set_pixel(x, y, c);
            }
        }

        // subtle grid pattern
        for y in (0..(SCREEN_H - TASKBAR_H) as i32).step_by(40) {
            for x in 0..SCREEN_W as i32 {
                let cur = fb.get_pixel(x, y);
                fb.set_pixel(x, y, Color::rgb(
                    cur.r.saturating_add(3),
                    cur.g.saturating_add(3),
                    cur.b.saturating_add(5),
                ));
            }
        }

        // draw desktop icons
        for icon in &self.icons {
            let r = icon.rect;

            // icon background
            fb.fill_rect(r, Color::rgb(0, 0, 0));
            fb.draw_rect_outline(r, Color::ACCENT, 1);

            // icon inner symbol (different per type)
            let sym_color = Color::ACCENT;
            match &icon.action {
                DesktopAction::OpenTerminal => {
                    fb.draw_text(">_", r.x + 20, r.y + 22, sym_color);
                }
                DesktopAction::OpenFileManager => {
                    fb.fill_rect(Rect::new(r.x+12, r.y+16, 40, 30), Color::AMBER);
                    fb.fill_rect(Rect::new(r.x+12, r.y+12, 20, 8),  Color::AMBER);
                }
                DesktopAction::OpenAIAgent => {
                    fb.fill_circle(r.x + 32, r.y + 28, 14, sym_color);
                    fb.fill_circle(r.x + 32, r.y + 28, 10, Color::DARK_GRAY);
                    fb.fill_circle(r.x + 32, r.y + 28,  4, sym_color);
                }
                DesktopAction::OpenAbout => {
                    fb.draw_text("i", r.x + 28, r.y + 18, sym_color);
                    fb.draw_line(r.x+32, r.y+30, r.x+32, r.y+48, sym_color);
                }
            }

            // icon label below
            let label_x = r.x + (r.w as i32 - icon.label.len() as i32 * 6) / 2;
            fb.draw_text(&icon.label, label_x, r.y + r.h as i32 + 4, Color::WHITE);
        }
    }
}
