use sdl3::{event::Event, pixels::Color, rect::Rect};
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
}

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context
		.video()
		.unwrap();

	let window = video_subsystem
		.window("sdl3-test", 800, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

	canvas.set_draw_color(Color::RGB(255, 0, 0));
	canvas.clear();
	canvas.present();

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let mut cube_pos = Vec2::new(0.0, 0.0);

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);

	let tick_time = Duration::from_secs_f64(1.0 / TICK_RATE);

	let mut ticks: u64 = 0;

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
			ticks += 1;
			update_tick(ticks, &mut cube_pos);
			accumulator -= tick_time;
		}

		canvas.set_draw_color(Color::RGB(255, 0, 0));
		canvas.clear();

		canvas.set_draw_color(Color::RGB(0, 255, 0));
		let rect = Rect::new(cube_pos.x as i32, cube_pos.y as i32, 100, 100);
		canvas
			.fill_rect(rect)
			.unwrap();

		canvas.present();
	}
}

fn update_tick(ticks: u64, cube_pos: &mut Vec2) {
	cube_pos.x = (ticks as f64 / 100.0).cos() * 350.0 + 350.0;
	cube_pos.y = (ticks as f64 / 100.0).sin() * 250.0 + 250.0;
}
