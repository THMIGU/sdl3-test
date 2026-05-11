use sdl3::{event::Event, pixels::Color};

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

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {
					..
				} => {
					break 'running;
				}
				_ => {}
			}
		}
	}
}
