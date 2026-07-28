//! macroquad frontend: input in, circles out.
//!
//! Everything that decides what happens lives in [`game_template::sim`]. This
//! file only translates between the window and the sim's logical world.

use macroquad::color::{Color, hsl_to_rgb};
use macroquad::input::{
    KeyCode, MouseButton, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed,
    mouse_position,
};
use macroquad::shapes::draw_circle;
use macroquad::text::draw_text;
use macroquad::time::get_frame_time;
use macroquad::window::{Conf, clear_background, next_frame, screen_height, screen_width};

use game_template::VERSION;
use game_template::sim::{InputFrame, Sim, Vec2, WORLD_H, WORLD_W};

const BACKGROUND: Color = Color::new(0.05, 0.05, 0.07, 1.0);
const OVERLAY: Color = Color::new(0.85, 0.85, 0.90, 0.75);
const TEXT_SIZE: f32 = 18.0;
const TEXT_MARGIN: f32 = 12.0;

fn window_conf() -> Conf {
    Conf {
        window_title: env!("CARGO_PKG_NAME").to_owned(),
        window_width: WORLD_W as i32,
        window_height: WORLD_H as i32,
        high_dpi: true,
        ..Conf::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sim = Sim::new(fresh_seed());

    loop {
        let view = View::fit(screen_width(), screen_height());
        let input = gather_input(&view);
        sim.advance(get_frame_time(), &input);
        draw(&sim, &view);
        next_frame().await;
    }
}

/// Wall-clock seed. Swap in `quad-url` here if you want seeds that survive in
/// a shareable URL fragment (see the docs).
fn fresh_seed() -> u64 {
    macroquad::miniquad::date::now().to_bits()
}

/// Letterboxed mapping between the sim's fixed logical world and the window.
struct View {
    scale: f32,
    origin: Vec2,
}

impl View {
    fn fit(screen_w: f32, screen_h: f32) -> Self {
        // A minimised window or hidden tab reports zero size; the floor keeps
        // the inverse mapping from producing infinities.
        let scale = (screen_w / WORLD_W)
            .min(screen_h / WORLD_H)
            .max(f32::EPSILON);
        Self {
            scale,
            origin: Vec2::new(
                WORLD_W.mul_add(-scale, screen_w) * 0.5,
                WORLD_H.mul_add(-scale, screen_h) * 0.5,
            ),
        }
    }

    fn to_screen(&self, world: Vec2) -> Vec2 {
        world * self.scale + self.origin
    }

    fn to_world(&self, screen: Vec2) -> Vec2 {
        (screen - self.origin) * self.scale.recip()
    }
}

fn gather_input(view: &View) -> InputFrame {
    let (mouse_x, mouse_y) = mouse_position();
    InputFrame {
        pointer: view.to_world(Vec2::new(mouse_x, mouse_y)),
        attract: is_mouse_button_down(MouseButton::Left),
        burst: is_mouse_button_pressed(MouseButton::Left),
        toggle_pause: is_key_pressed(KeyCode::Space),
        reseed: is_key_pressed(KeyCode::R).then(fresh_seed),
    }
}

fn draw(sim: &Sim, view: &View) {
    clear_background(BACKGROUND);

    let alpha = sim.alpha();
    for particle in sim.particles() {
        let pos = view.to_screen(particle.interpolated(alpha));
        draw_circle(
            pos.x,
            pos.y,
            particle.radius * view.scale,
            hsl_to_rgb(particle.hue, 0.65, 0.62),
        );
    }

    let state = if sim.is_paused() { "paused" } else { "running" };
    let lines = [
        format!("{} {VERSION}", env!("CARGO_PKG_NAME")),
        format!("seed {:016x} - {state}", sim.seed()),
        "drag: attract | click: burst | space: pause | R: reseed".to_owned(),
    ];
    for (row, line) in lines.iter().enumerate() {
        let baseline = TEXT_SIZE.mul_add(row as f32, TEXT_MARGIN + TEXT_SIZE);
        draw_text(line, TEXT_MARGIN, baseline, TEXT_SIZE, OVERLAY);
    }
}
