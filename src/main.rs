#![windows_subsystem = "windows"]

mod assets;
mod ball;
mod game;
mod vec2;

use crate::assets::Assets;
use crate::game::Game;
use sdl3::{event::Event, pixels::Color, rect::Rect};
use std::time::{Duration, Instant};

const TICK_RATE: f64 = 120.0;
const W_WIDTH: u32 = 600;
const W_HEIGHT: u32 = 600;

fn main() {
	let mut game = Game::new();
	game.ball
		.vel
		.set(2.5, 1.5);

	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context
		.video()
		.unwrap();

	let window = video_subsystem
		.window("Pong", W_WIDTH, W_HEIGHT)
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
			.fill_rect(Rect::new(298, 0, 4, 600))
			.unwrap();

		canvas
			.copy(&assets.ball_texture, None, game.ball.hitbox)
			.unwrap();

		canvas.present();
	}
}
