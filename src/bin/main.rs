const PIXEL_WIDTH: usize = 800;
const PIXEL_HEIGHT: usize = 600;
const PARTICLE_NUMBER: usize = 2000;
const SCALING: f64 = 2.;
const WIDTH: f64 = PIXEL_WIDTH as f64 / SCALING;
const HEIGHT: f64 = PIXEL_HEIGHT as f64 / SCALING;

use minifb::{Window, WindowOptions};
use particle::{Particle, World};
use piet::{
    kurbo::{Affine, Circle},
    Color, ImageBuf, RenderContext, Text, TextLayoutBuilder,
};
use piet_common::Device;
// use quad_tree::QuadTree;
use rstar_tree::RStartree;
use tree_drawings::{DrawContext, Drawable};
// use zorder_tree::ZOrderTree;
use std::error::Error;
use v2::{TreeValue, V2};

type WorldType = World<RStartree<Particle>>;

fn main() -> Result<(), Box<dyn Error>> {
    draw_app()
}

fn draw_app() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new(
        "Test - ESC to exit",
        PIXEL_WIDTH,
        PIXEL_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let rng = || rand::random::<f64>();
    let mut world: WorldType = World::new(V2::new(WIDTH, HEIGHT), V2::new(0., 100.));
    world.add_random_particles(PARTICLE_NUMBER, rng);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut device = Box::new(Device::new()?);
    while window.is_open() {
        let mouse_pos = window
            .get_mouse_pos(minifb::MouseMode::Discard)
            .and_then(|(x, y)| {
                let x = x as f64 / SCALING;
                let y = y as f64 / SCALING;
                Some(V2::new(x, y))
            });
        let is_pressing = window.get_mouse_down(minifb::MouseButton::Left);
        world.is_pressing_mouse = is_pressing;
        let q_pressed = window.is_key_released(minifb::Key::W);
        if q_pressed {
            world.show_quad_tree = !world.show_quad_tree;
        }
        let mut target = device.bitmap_target(PIXEL_WIDTH, PIXEL_HEIGHT, 1.)?;
        {
            let mut piet_context = target.render_context();
            let evolve_start = std::time::Instant::now();
            world.evolve(4);
            let evolve_duration = evolve_start.elapsed();

            let txt = piet_context
                .text()
                .new_text_layout(format!("Evolve: {:?}", evolve_duration))
                .text_color(Color::WHITE)
                .build()
                .unwrap();
            piet_context.draw_text(&txt, (10., 10.));
            piet_context.transform(Affine::scale(SCALING));
            let draw_context = DrawContext { mouse_pos };
            world.draw(&mut piet_context, &draw_context);
            piet_context.finish();
        };
        let drawing = buff_to_vec(target.to_image_buf(piet::ImageFormat::RgbaPremul)?);
        window
            .update_with_buffer(&drawing, PIXEL_WIDTH, PIXEL_HEIGHT)
            .unwrap();
    }
    Ok(())
}

fn buff_to_vec(buff: ImageBuf) -> Vec<u32> {
    let drawing = buff
        .raw_pixels()
        .chunks_exact(4)
        .map(|chunk| {
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            let a = chunk[3];
            let color = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            color
        })
        .collect::<Vec<u32>>();
    drawing
}
