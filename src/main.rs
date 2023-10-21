const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct GraphicsContext {
    device: Device,
}

impl GraphicsContext {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let device = Device::new()?;
        Ok(GraphicsContext { device })
    }

    pub fn with_context(
        &mut self,
        mut f: impl FnMut(&mut D2DRenderContext) -> Result<(), Box<dyn Error>>,
    ) -> Result<Vec<u32>, Box<dyn Error>> {
        let mut target = self.device.bitmap_target(WIDTH, HEIGHT, 1.)?;
        {
            let mut piet_context = target.render_context();
            (f)(&mut piet_context)?;
        }

        let buff = target.to_image_buf(piet::ImageFormat::RgbaPremul)?;
        let drawing = buff
            .raw_pixels()
            .chunks_exact(4)
            .map(|chunk| {
                let r = chunk[0];
                let g = chunk[1];
                let b = chunk[2];
                let a = chunk[3];
                let color =
                    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                color
            })
            .collect::<Vec<u32>>();
        Ok(drawing)
    }
}

use std::{error::Error, time::Duration};

use gui::run_sample;
use minifb::{Window, WindowOptions};
use piet_common::Device;
use piet_direct2d::D2DRenderContext;

fn main() -> Result<(), Box<dyn Error>> {
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
    let mut gc = GraphicsContext::new()?;
    let drawing = gc.with_context(|r| run_sample(r))?;
    window.limit_update_rate(Some(Duration::from_millis(16)));
    while window.is_open() {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&drawing, WIDTH, HEIGHT).unwrap();
        println!("drawing");
    }
    Ok(())
}
