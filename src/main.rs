use sdl3::{event::Event, pixels::Color};
use std::thread::sleep;
use std::time::Duration;

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

	let mut frames: u128 = 0;

	'running: loop {
		frames += 1;

		let t = frames as f32 * 0.02;

		let r = ((t.sin() * 0.5 + 0.5) * 255.0) as u8;
		let g = (((t + 2.094).sin() * 0.5 + 0.5) * 255.0) as u8;
		let b = (((t + 4.188).sin() * 0.5 + 0.5) * 255.0) as u8;

		canvas.set_draw_color(Color::RGB(r, g, b));
		canvas.clear();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {
					..
				} => break 'running,
				_ => {}
			}
		}

		canvas.present();

		sleep(Duration::from_millis(1000 / 60));
	}
}
