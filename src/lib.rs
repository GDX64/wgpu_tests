use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};
pub struct App<V: Component> {
    comp: V,
}

impl<V: Component> App<V> {
    pub fn draw(&mut self, piet_context: &mut impl RenderContext) {
        self.comp.update(piet_context);
        self.comp.draw(piet_context);
        piet_context.finish().unwrap();
    }
}

pub fn create_new_app() -> App<impl Component> {
    App {
        comp: Button::new("Hello, world!".to_string()),
    }
}

pub trait Component {
    fn draw(&self, piet_context: &mut impl RenderContext);
    fn update(&mut self, piet_context: &mut impl RenderContext) {}
}

struct Button {
    text: String,
}

impl Button {
    fn new(text: String) -> Self {
        Self { text }
    }
}

impl Component for Button {
    fn draw(&self, piet_context: &mut impl RenderContext) {
        let txt = piet_context
            .text()
            .new_text_layout(self.text.clone())
            .font(FontFamily::SYSTEM_UI, 12.0)
            .text_color(Color::BLUE)
            .build()
            .ok();
        if let Some(txt) = txt {
            piet_context.draw_text(&txt, (100., 100.));
        }
    }

    fn update(&mut self, piet_context: &mut impl RenderContext) {}
}
