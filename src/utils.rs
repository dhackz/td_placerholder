use ggez::mint::Point2;

/// Represents the x and y scale used to map in-game coordinates with viewport.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

impl Scale {
    pub fn to_game_point(self, x: f32, y: f32) -> Point2<f32> {
        Point2 {
            x: x / self.x,
            y: y / self.y,
        }
    }

    pub fn to_viewport_point(self, x: f32, y: f32) -> Point2<f32> {
        Point2 {
            x: x * self.x,
            y: y * self.y,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}
