const PIXEL_WIDTH: usize = 800;
const PIXEL_HEIGHT: usize = 600;
const PARTICLE_NUMBER: usize = 5000;
const SCALING: f64 = 2.;
const WIDTH: f64 = PIXEL_WIDTH as f64 / SCALING;
const HEIGHT: f64 = PIXEL_HEIGHT as f64 / SCALING;
mod quad_tree;

use minifb::{Window, WindowOptions};
use particle::{World, PARTICLE_RADIUS, V2};
use piet::{
    kurbo::{Affine, Circle, Line, Rect},
    Color, ImageBuf, RenderContext, Text, TextLayoutBuilder,
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
        let mouse_pos = window
            .get_mouse_pos(minifb::MouseMode::Discard)
            .map(|(x, y)| {
                let x = x as f64 / SCALING;
                let y = y as f64 / SCALING;
                V2::new(x, y)
            });
        let mut target = device.bitmap_target(PIXEL_WIDTH, PIXEL_HEIGHT, 1.)?;
        {
            let mut piet_context = target.render_context();
            let evolve_start = std::time::Instant::now();
            if let None = mouse_pos {
                world.evolve();
            }
            let evolve_duration = evolve_start.elapsed();
            let txt = piet_context
                .text()
                .new_text_layout(format!("Evolve: {:?}", evolve_duration))
                .text_color(Color::WHITE)
                .build()
                .unwrap();
            piet_context.draw_text(&txt, (10., 10.));
            draw(&mut piet_context, &world, mouse_pos);
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

fn draw(piet_context: &mut impl piet::RenderContext, world: &World, mouse_pos: Option<V2>) {
    let brush = piet_context.solid_brush(Color::WHITE);
    piet_context.transform(Affine::scale(SCALING));
    world.particles.iter().for_each(|particle| {
        let x = particle.position.x;
        let y = particle.position.y;
        let particle = Circle::new((x, y), 1.);
        piet_context.fill(particle, &brush);
    });
    // let center = V2::new(WIDTH as f64 / 2., HEIGHT as f64 / 2.);
    // let gradient = world.calc_gradient(&center);
    // draw_arrow(&center, &center.add(&gradient), piet_context);
    draw_tree(piet_context, &world, mouse_pos);
    piet_context.finish().unwrap();
}

fn draw_arrow(p1: &V2, p2: &V2, piet_context: &mut impl piet::RenderContext) {
    let brush = piet_context.solid_brush(Color::WHITE);
    let line = Line::new((p1.x, p1.y), (p2.x, p2.y));
    piet_context.stroke(line, &brush, 1.0);
    let rect = Rect::new(p2.x - 5., p2.y - 5., p2.x + 5., p2.y + 5.);
    piet_context.fill(rect, &brush);
}

fn draw_tree(
    piet_context: &mut impl piet::RenderContext,
    world: &World,
    mouse_pos: Option<V2>,
) -> Option<()> {
    let mouse_pos = mouse_pos?;
    let brush = piet_context.solid_brush(Color::WHITE);
    let mut tree = quad_tree::QuadTree::new(V2::new(0., 0.), WIDTH, HEIGHT);
    world.particles.iter().for_each(|particle| {
        tree.insert(particle.clone());
    });
    tree.for_each(&mut |n| {
        let rect = n.get_rect();
        piet_context.stroke(rect, &brush, 1.0);
    });
    let query_circ = Circle::new((mouse_pos.x, mouse_pos.y), PARTICLE_RADIUS);
    piet_context.stroke(query_circ, &Color::RED, 1.0);
    tree.query_distance(
        &V2::new(query_circ.center.x, query_circ.center.y),
        query_circ.radius,
        |p| {
            let rect = Circle::new((p.position.x, p.position.y), 1.);
            piet_context.fill(rect, &Color::RED);
        },
    );
    Some(())
}

impl quad_tree::TreeValue for particle::Particle {
    fn position(&self) -> V2 {
        self.position.clone()
    }
}
