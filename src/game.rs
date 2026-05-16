use crate::ball::Ball;
use crate::vec2::Vec2;
use sdl3::rect::Rect;

pub struct Game {
	pub ticks: u64,
	pub ball: Ball,
	pub walls: Vec<Rect>,
}

impl Game {
	pub fn new() -> Self {
		Self {
			ticks: 0,
			ball: Ball::new(),
			walls: vec![
				Rect::new(0, -100, 600, 100),
				Rect::new(0, 600, 600, 100),
				Rect::new(-100, 0, 100, 600),
				Rect::new(600, 0, 100, 600),
			],
		}
	}

	pub fn update(&mut self) {
		self.ticks += 1;

		let mut next = self
			.ball
			.pos
			.clone();
		next.add_assign(self.ball.vel);

		self.ball
			.hitbox
			.center_on(next.as_point());

		for wall in &self.walls {
			if self
				.ball
				.hitbox
				.has_intersection(*wall)
			{
				let x = -wall.x.signum();
				let y = -wall.y.signum();

				let normal = Vec2::new(x as f64, y as f64);

				self.ball.vel = self
					.ball
					.vel
					.reflect(normal);
			}
		}

		self.ball
			.pos
			.add_assign(self.ball.vel);

		self.ball
			.hitbox
			.center_on(
				self.ball
					.pos
					.as_point(),
			);
	}
}
