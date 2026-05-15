#![windows_subsystem = "windows"]

mod assets;
mod ball;
mod vec2;

use assets::Assets;
use ball::Ball;
use sdl3::{event::Event, pixels::Color, rect::Rect};
use std::time::{Duration, Instant};

use crate::vec2::Vec2;

const TICK_RATE: f64 = 120.0;
const W_WIDTH: u32 = 600;
const W_HEIGHT: u32 = 600;

struct Game {
	ticks: u64,
	ball: Ball,
	walls: Vec<Rect>,
}

impl Game {
	fn new() -> Self {
		Self {
			ticks: 0,
			ball: Ball::new(),
			walls: vec![
				Rect::new(0, -1, 600, 1),
				Rect::new(0, 601, 600, 1),
				Rect::new(-1, 0, 1, 600),
				Rect::new(601, 0, 1, 600),
			],
		}
	}

	fn update(&mut self) {
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
				let x = wall.x.signum();
				let y = wall.y.signum();

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

fn main() {
	let mut game = Game::new();
	game.ball
		.vel
		.set(1.5, 1.0);

	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context
		.video()
		.unwrap();

	let window = video_subsystem
		.window("sdl3-test", W_WIDTH, W_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

	let texture_creator = canvas.texture_creator();
	let assets = Assets::new(&texture_creator);

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);
	let tick_time = Duration::from_secs_f64(1.0 / TICK_RATE);

	'running: loop {
		let now = Instant::now();
		accumulator += now.duration_since(last_frame);
		last_frame = now;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {
					..
				} => break 'running,
				_ => {}
			}
		}

		while accumulator >= tick_time {
			game.update();
			accumulator -= tick_time;
		}

		canvas.set_draw_color(Color::BLACK);
		canvas.clear();

		canvas.set_draw_color(Color::WHITE);

		canvas
			.copy(&assets.ball_texture, None, game.ball.hitbox)
			.unwrap();

		canvas.present();
	}
}
