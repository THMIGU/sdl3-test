use sdl3::rect::Point;

#[derive(Clone, Copy)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
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

	pub fn add(self, other: Vec2) -> Vec2 {
		Vec2::new(self.x + other.x, self.y + other.y)
	}

	pub fn add_assign(&mut self, other: Vec2) -> &mut Self {
		self.x += other.x;
		self.y += other.y;
		self
	}

	pub fn sub(self, other: Vec2) -> Vec2 {
		Vec2::new(self.x - other.x, self.y - other.y)
	}

	pub fn scl(self, scl: f64) -> Vec2 {
		Vec2::new(self.x * scl, self.y * scl)
	}

	pub fn dot(self, other: Vec2) -> f64 {
		self.x * other.x + self.y * other.y
	}

	pub fn reflect(self, normal: Vec2) -> Vec2 {
		let d = self.dot(normal);
		self.sub(normal.scl(2.0 * d))
	}

	pub fn as_point(self) -> Point {
		Point::new(self.x as i32, self.y as i32)
	}
}
