use crate::vec2::Vec2;
use sdl3::rect::{Point, Rect};

pub struct Paddle {
	pub pos: Vec2,
	pub vel: Vec2,
	pub hitbox: Rect,
}

impl Paddle {
	pub fn new(x: f64, y: f64) -> Self {
		Self {
			pos: Vec2::new(x, y),
			vel: Vec2::new(0.0, 0.0),
			hitbox: Rect::from_center(Point::new(x as i32, y as i32), 25, 100),
		}
	}
}
