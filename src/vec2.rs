use sdl3::rect::Point;

pub struct Vec2 {
	x: f64,
	y: f64,
}

impl Vec2 {
	pub fn new(x: f64, y: f64) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn set(&mut self, x: f64, y: f64) -> &mut Self {
		self.x = x;
		self.y = y;
		self
	}

	pub fn add(&mut self, other: &Vec2) -> &mut Self {
		self.x += other.x;
		self.y += other.y;
		self
	}

	pub fn as_point(&self) -> Point {
		Point::new(self.x as i32, self.y as i32)
	}
}
