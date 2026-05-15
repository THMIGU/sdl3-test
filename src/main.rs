#![windows_subsystem = "windows"]

use sdl3::{
	event::Event,
	image::LoadTexture,
	pixels::Color,
	rect::{Point, Rect},
	render::{Texture, TextureCreator},
	video::WindowContext,
};
use std::time::{Duration, Instant};

const TICK_RATE: f64 = 120.0;
const W_WIDTH: u32 = 600;
const W_HEIGHT: u32 = 600;

struct Vec2 {
	x: f64,
	y: f64,
}

impl Vec2 {
	fn new(x: f64, y: f64) -> Self {
		Self {
			x,
			y,
		}
	}

	fn set(&mut self, x: f64, y: f64) -> &mut Self {
		self.x = x;
		self.y = y;
		self
	}

	fn add(&mut self, other: &Vec2) -> &mut Self {
		self.x += other.x;
		self.y += other.y;
		self
	}

	fn as_point(&self) -> Point {
		Point::new(self.x as i32, self.y as i32)
	}
}

struct Ball {
	pos: Vec2,
	vel: Vec2,
	hitbox: Rect,
}

impl Ball {
	fn new() -> Self {
		Self {
			pos: Vec2::new(0.0, 0.0),
			vel: Vec2::new(0.0, 0.0),
			hitbox: Rect::from_center(Point::new(0, 0), 25, 25),
		}
	}
}

struct Game {
	ticks: u64,
	ball: Ball,
}

struct Assets<'a> {
	ball_texture: Texture<'a>,
}

impl<'a> Assets<'a> {
	fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
		Self {
			ball_texture: texture_creator
				.load_texture("assets/ball.png")
				.unwrap(),
		}
	}
}

impl Game {
	fn new() -> Self {
		Self {
			ticks: 0,
			ball: Ball::new(),
		}
	}

	fn update(&mut self) {
		self.ticks += 1;

		self.ball
			.pos
			.add(&self.ball.vel);
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
		.set(1.5, 1.5);

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
