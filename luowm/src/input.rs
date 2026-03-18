use crate::types::Point;

#[derive(Debug, Clone)]
pub enum InputEvent {
    MouseMove(Point),
    MouseDown(Point, MouseButton),
    MouseUp(Point, MouseButton),
    KeyDown(Key),
    KeyUp(Key),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Escape,
    Tab,
    Up,
    Down,
    Left,
    Right,
    F1, F2, F3, F4,
    Unknown,
}

/// Input state machine — tracks mouse position and button state
pub struct InputState {
    pub mouse_pos:    Point,
    pub left_down:    bool,
    pub right_down:   bool,
    pub drag_start:   Option<Point>,
    pub drag_active:  bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_pos:   Point { x: 512, y: 384 },
            left_down:   false,
            right_down:  false,
            drag_start:  None,
            drag_active: false,
        }
    }

    pub fn process(&mut self, ev: &InputEvent) {
        match ev {
            InputEvent::MouseMove(p) => {
                self.mouse_pos = *p;
                if self.left_down {
                    if let Some(start) = self.drag_start {
                        let dx = (p.x - start.x).abs();
                        let dy = (p.y - start.y).abs();
                        if dx > 4 || dy > 4 {
                            self.drag_active = true;
                        }
                    }
                }
            }
            InputEvent::MouseDown(p, MouseButton::Left) => {
                self.left_down  = true;
                self.drag_start = Some(*p);
                self.drag_active = false;
            }
            InputEvent::MouseUp(_, MouseButton::Left) => {
                self.left_down  = false;
                self.drag_start = None;
                self.drag_active = false;
            }
            InputEvent::MouseDown(_, MouseButton::Right) => {
                self.right_down = true;
            }
            InputEvent::MouseUp(_, MouseButton::Right) => {
                self.right_down = false;
            }
            _ => {}
        }
    }

    pub fn delta(&self, prev: Point) -> (i32, i32) {
        (self.mouse_pos.x - prev.x, self.mouse_pos.y - prev.y)
    }
}

/// Cursor renderer
pub fn draw_cursor(fb: &mut crate::framebuffer::Framebuffer, p: Point) {
    use crate::types::Color;
    // arrow cursor shape
    let pts: &[(i32,i32)] = &[
        (0,0),(1,0),(2,0),(3,0),(4,0),(5,0),(6,0),(7,0),(8,0),(9,0),(10,0),
        (0,1),(1,1),(2,1),(3,1),(4,1),(5,1),
        (0,2),(1,2),(2,2),(3,2),(4,2),
        (0,3),(1,3),(2,3),(3,3),(4,3),(5,3),
        (0,4),(1,4),(5,4),(6,4),
        (0,5),(6,5),(7,5),
        (0,6),(7,6),(8,6),
        (0,7),(8,7),(9,7),
        (0,8),(9,8),(10,8),
    ];
    for (dx, dy) in pts {
        fb.set_pixel(p.x + dx, p.y + dy, Color::WHITE);
    }
    // outline
    for (dx, dy) in pts {
        fb.set_pixel(p.x + dx - 1, p.y + dy, Color::BLACK);
        fb.set_pixel(p.x + dx + 1, p.y + dy, Color::BLACK);
        fb.set_pixel(p.x + dx, p.y + dy - 1, Color::BLACK);
        fb.set_pixel(p.x + dx, p.y + dy + 1, Color::BLACK);
    }
    // redraw white on top of outline
    for (dx, dy) in pts {
        fb.set_pixel(p.x + dx, p.y + dy, Color::WHITE);
    }
}
