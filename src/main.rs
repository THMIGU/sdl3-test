use sdl3::{event::Event, pixels::Color};
use std::time::{Duration, Instant};

const TICK_RATE: f64 = 120.0;

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
}

struct Ball {
	pos: Vec2,
	vel: Vec2,
}

impl Ball {
	fn new() -> Self {
		Self {
			pos: Vec2::new(0.0, 0.0),
			vel: Vec2::new(0.0, 0.0),
		}
	}
}

struct Game {
	ticks: u64,
	ball: Ball,
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
	}
}

fn main() {
	let mut game = Game::new();
	game.ball
		.vel
		.set(1.0, 0.0);

	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context
		.video()
		.unwrap();

	let window = video_subsystem
		.window("sdl3-test", 600, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

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

		canvas.present();
	}
}
