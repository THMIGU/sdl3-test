#![windows_subsystem = "windows"]

mod assets;
mod ball;
mod game;
mod paddle;
mod vec2;

use crate::assets::Assets;
use crate::game::Game;
use sdl3::{
	event::Event,
	pixels::Color,
	rect::{Point, Rect},
	ttf,
};
use std::time::{Duration, Instant};

const TICK_RATE: f64 = 120.0;
const W_WIDTH: u32 = 600;
const W_HEIGHT: u32 = 600;

fn main() {
	let mut game = Game::new();
	game.ball
		.vel
		.set(-2.5, 0.0);

	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context
		.video()
		.unwrap();

	let ttf_context = ttf::init().unwrap();

	let window = video_subsystem
		.window("Pong", W_WIDTH, W_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

	let texture_creator = canvas.texture_creator();
	let assets = Assets::new(&texture_creator);

	let font = ttf_context
		.load_font("assets/bit5x3.ttf", 64.0)
		.unwrap();

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
			let keyboard = event_pump.keyboard_state();

			game.update(&keyboard);
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

		canvas
			.copy(&assets.paddle_texture, None, game.paddle_l.hitbox)
			.unwrap();
		canvas
			.copy(&assets.paddle_texture, None, game.paddle_r.hitbox)
			.unwrap();

		let font_surface = font
			.render(&format!("{} {}", game.score_l, game.score_r))
			.blended(Color::WHITE)
			.unwrap();

		let font_texture = texture_creator
			.create_texture_from_surface(&font_surface)
			.unwrap();

		let font_target =
			Rect::from_center(Point::new(304, 35), font_surface.width(), font_surface.height());

		canvas
			.copy(&font_texture, None, font_target)
			.unwrap();

		canvas.present();
	}
}
