use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
struct CanvasDriven {
    canvas: HtmlCanvasElement,
    world: World<RStartree<Particle>>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[wasm_bindgen]
impl CanvasDriven {
    pub fn new(canvas: HtmlCanvasElement) -> CanvasDriven {
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let mut world = World::new(V2::new(width, height), V2::new(0., 10.));
        world.add_random_particles(100, random);
        CanvasDriven { canvas, world }
    }

    pub fn evolve(&mut self) {
        self.world.evolve(4);
    }

    pub fn draw(&self) {
        self._draw();
    }
}

impl CanvasDriven {
    fn _draw(&self) -> Option<()> {
        let ctx = self.canvas.get_context("2d").ok()??;
        let win = window()?;
        let ctx: CanvasRenderingContext2d = ctx.dyn_into().ok()?;
        ctx.clear_rect(0., 0., 1000., 1000.);
        let mut piet_ctx = WebRenderContext::new(ctx, win);
        let draw_context = DrawContext { mouse_pos: None };
        self.world.draw(&mut piet_ctx, &draw_context);
        piet_ctx.finish().ok()?;
        Some(())
    }
}
