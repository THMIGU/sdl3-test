use crate::vec2::Vec2;
use sdl3::rect::{Point, Rect};

pub struct Ball {
	pub pos: Vec2,
	pub vel: Vec2,
	pub hitbox: Rect,
}

impl Ball {
	pub fn new() -> Self {
		Self {
			pos: Vec2::new(300.0, 300.0),
			vel: Vec2::new(0.0, 0.0),
			hitbox: Rect::from_center(Point::new(300, 300), 25, 25),
		}
	}
}
