/// Core types shared across all luowm modules

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x
            && p.x < self.x + self.w as i32
            && p.y >= self.y
            && p.y < self.y + self.h as i32
    }
    pub fn titlebar(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, 24)
    }
    pub fn close_btn(&self) -> Rect {
        Rect::new(self.x + self.w as i32 - 20, self.y + 4, 16, 16)
    }
    pub fn min_btn(&self) -> Rect {
        Rect::new(self.x + self.w as i32 - 40, self.y + 4, 16, 16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    pub const BLACK:       Color = Color::rgb(0,   0,   0);
    pub const WHITE:       Color = Color::rgb(255, 255, 255);
    pub const DARK_GRAY:   Color = Color::rgb(30,  30,  30);
    pub const GRAY:        Color = Color::rgb(60,  60,  60);
    pub const LIGHT_GRAY:  Color = Color::rgb(200, 200, 200);
    pub const ACCENT:      Color = Color::rgb(0,   120, 215);
    pub const ACCENT_DARK: Color = Color::rgb(0,   90,  160);
    pub const RED:         Color = Color::rgb(196, 43,  28);
    pub const GREEN:       Color = Color::rgb(16,  124, 16);
    pub const AMBER:       Color = Color::rgb(202, 80,  16);
    pub const TITLEBAR:    Color = Color::rgb(40,  40,  40);
    pub const TITLEBAR_FOCUS: Color = Color::rgb(0, 100, 180);
    pub const DESKTOP_BG: Color = Color::rgb(15,  15,  25);
    pub const TASKBAR_BG: Color = Color::rgb(20,  20,  20);
}
