const PIXEL_WIDTH: usize = 300;
const PIXEL_HEIGHT: usize = 300;
const PARTICLE_NUMBER: usize = 100;
const SCALING: f64 = 5.;
const WIDTH: f64 = PIXEL_WIDTH as f64 / SCALING;
const HEIGHT: f64 = PIXEL_HEIGHT as f64 / SCALING;

use minifb::{Window, WindowOptions};
use particle::{World, V2};
use piet::{
    kurbo::{Affine, Circle, Line, Rect},
    Color, ImageBuf,
};
use piet_common::Device;
use std::error::Error;
mod particle;

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
    let mut world = World::new(V2::new(WIDTH, HEIGHT), V2::new(0., 9.));
    world.add_random_particles(PARTICLE_NUMBER, rng);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut device = Box::new(Device::new()?);
    while window.is_open() {
        let mut target = device.bitmap_target(PIXEL_WIDTH, PIXEL_HEIGHT, 1.)?;
        {
            let mut piet_context = target.render_context();
            world.evolve();
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

fn draw(piet_context: &mut impl piet::RenderContext, world: &World) {
    let brush = piet_context.solid_brush(Color::WHITE);
    piet_context.transform(Affine::scale(SCALING));
    world.particles.iter().for_each(|particle| {
        let x = particle.position.x;
        let y = particle.position.y;
        let rect = Circle::new((x, y), 1.);
        piet_context.fill(rect, &brush);
    });
    // let center = V2::new(WIDTH as f64 / 2., HEIGHT as f64 / 2.);
    // let gradient = world.calc_gradient(&center);
    // draw_arrow(&center, &center.add(&gradient), piet_context);
    piet_context.finish().unwrap();
}

fn draw_arrow(p1: &V2, p2: &V2, piet_context: &mut impl piet::RenderContext) {
    let brush = piet_context.solid_brush(Color::WHITE);
    let line = Line::new((p1.x, p1.y), (p2.x, p2.y));
    piet_context.stroke(line, &brush, 1.0);
    let rect = Rect::new(p2.x - 5., p2.y - 5., p2.x + 5., p2.y + 5.);
    piet_context.fill(rect, &brush);
}
