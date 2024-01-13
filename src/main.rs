const WIDTH: usize = 640;
const HEIGHT: usize = 360;
use minifb::{Window, WindowOptions};
use piet::{kurbo::Rect, Color, ImageBuf};
use piet_common::Device;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    draw_app()
}

fn draw_app() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut device = Box::new(Device::new()?);
    while window.is_open() {
        let mut target = device.bitmap_target(WIDTH, HEIGHT, 1.)?;
        {
            let mut piet_context = target.render_context();
            draw(&mut piet_context);
        };
        let drawing = buff_to_vec(target.to_image_buf(piet::ImageFormat::RgbaPremul)?);
        window.update_with_buffer(&drawing, WIDTH, HEIGHT).unwrap();
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

fn draw(piet_context: &mut impl piet::RenderContext) {
    let rect = Rect::new(0., 0., 200., 200.);
    let brush = piet_context.solid_brush(Color::RED);
    piet_context.fill(rect, &brush);
    piet_context.finish();
}
