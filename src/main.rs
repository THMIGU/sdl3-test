use sdl3::{event::Event, pixels::Color};
use std::time::{Duration, Instant};

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

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);

	let tick_time = Duration::from_secs_f64(1.0 / 120.0);

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
			println!("Tick!");
			accumulator -= tick_time;
		}

		canvas.clear();
		canvas.present();

		println!("Render!")
	}
}
