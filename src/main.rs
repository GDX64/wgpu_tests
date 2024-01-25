const PIXEL_WIDTH: usize = 800;
const PIXEL_HEIGHT: usize = 600;
const PARTICLE_NUMBER: usize = 1500;
const SCALING: f64 = 2.;
const WIDTH: f64 = PIXEL_WIDTH as f64 / SCALING;
const HEIGHT: f64 = PIXEL_HEIGHT as f64 / SCALING;
mod base_types;
mod quad_tree;
mod rstar_tree;
mod tree_drawings;
mod v2;
mod zorder_tree;

use minifb::{Window, WindowOptions};
use particle::{Particle, World};
use piet::{
    kurbo::{Affine, Circle},
    Color, ImageBuf, RenderContext, Text, TextLayoutBuilder,
};
use piet_common::Device;
// use quad_tree::QuadTree;
use rstar_tree::RStartree;
use tree_drawings::TreeDrawable;
// use zorder_tree::ZOrderTree;
use std::error::Error;
use v2::{TreeValue, V2};
mod particle;

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
    let mut world = World::new(V2::new(WIDTH, HEIGHT), V2::new(0., 100.));
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
            world.update_mouse_pos(mouse_pos);
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
            draw(&mut piet_context, &world);
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

fn draw(piet_context: &mut impl piet::RenderContext, world: &WorldType) {
    let brush = Color::WHITE;
    piet_context.transform(Affine::scale(SCALING));
    world.particles.iter().for_each(|particle| {
        let x = particle.position.x;
        let y = particle.position.y;
        let v = particle.velocity.len();
        let b = brush.with_alpha((v / 50.).min(1.).max(0.3));
        let particle = Circle::new((x, y), 2.);
        piet_context.fill(particle, &b);
    });
    // let center = V2::new(WIDTH as f64 / 2., HEIGHT as f64 / 2.);
    // let gradient = world.calc_gradient(&center);
    // draw_arrow(&center, &center.add(&gradient), piet_context);
    if let Some(ref mouse_pos) = world.mouse_pos {
        if world.show_quad_tree {
            world.tree.draw(piet_context, mouse_pos);
        }
    }
    piet_context.finish().unwrap();
}

impl TreeValue for particle::Particle {
    fn position(&self) -> V2 {
        self.position.clone()
    }

    fn offset_pos(&mut self) {
        self.position.x += 0.0001;
        self.position.y += 0.0001;
    }
}
