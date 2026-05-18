use crate::ball::Ball;
use crate::paddle::Paddle;
use crate::vec2::Vec2;
use sdl3::keyboard::{KeyboardState, Scancode};
use sdl3::rect::Rect;

pub struct Game {
	pub ticks: u64,
	pub ball: Ball,
	pub paddle_l: Paddle,
	pub paddle_r: Paddle,
	pub walls: Vec<Rect>,
	pub score_l: u64,
	pub score_r: u64,
}

impl Game {
	pub fn new() -> Self {
		Self {
			ticks: 0,
			ball: Ball::new(),
			paddle_l: Paddle::new(25.0, 300.0),
			paddle_r: Paddle::new(575.0, 300.0),
			walls: vec![
				Rect::new(0, -100, 600, 100),
				Rect::new(0, 600, 600, 100),
				Rect::new(-100, 0, 100, 600),
				Rect::new(600, 0, 100, 600),
			],
			score_l: 0,
			score_r: 0,
		}
	}

	pub fn update(&mut self, keyboard: &KeyboardState<'_>) {
		self.ticks += 1;

		self.paddle_l.vel.y *= 0.9;
		self.paddle_r.vel.y *= 0.9;

		let mut movement_l = 0.0;
		if keyboard.is_scancode_pressed(Scancode::W) {
			movement_l -= 3.0;
		}
		if keyboard.is_scancode_pressed(Scancode::S) {
			movement_l += 3.0;
		}

		let mut movement_r = 0.0;
		if keyboard.is_scancode_pressed(Scancode::Up) {
			movement_r -= 3.0;
		}
		if keyboard.is_scancode_pressed(Scancode::Down) {
			movement_r += 3.0;
		}

		self.paddle_l.vel.y = movement_l;
		self.paddle_r.vel.y = movement_r;

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

				if x == -1 {
					self.score_l += 1;
					self.ball
						.pos
						.set(300.0, 300.0);
					self.ball
						.vel
						.set(-2.5, 0.0);
				} else if x == 1 {
					self.score_r += 1;
					self.ball
						.pos
						.set(300.0, 300.0);
					self.ball
						.vel
						.set(2.5, 0.0);
				}
			}
		}

		if self.ball.vel.x < 0.0
			&& self
				.ball
				.hitbox
				.has_intersection(self.paddle_l.hitbox)
		{
			let normal = Vec2::new(1.0, 0.0);

			self.ball.vel = self
				.ball
				.vel
				.reflect(normal)
				.add(
					self.paddle_l
						.vel
						.scl(0.5),
				);
		} else if self.ball.vel.x > 0.0
			&& self
				.ball
				.hitbox
				.has_intersection(self.paddle_r.hitbox)
		{
			let normal = Vec2::new(-1.0, 0.0);

			self.ball.vel = self
				.ball
				.vel
				.reflect(normal)
				.add(
					self.paddle_r
						.vel
						.scl(0.5),
				);
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

		self.paddle_l
			.pos
			.add_assign(self.paddle_l.vel);
		self.paddle_l
			.hitbox
			.center_on(
				self.paddle_l
					.pos
					.as_point(),
			);

		self.paddle_r
			.pos
			.add_assign(self.paddle_r.vel);
		self.paddle_r
			.hitbox
			.center_on(
				self.paddle_r
					.pos
					.as_point(),
			);
	}
}
