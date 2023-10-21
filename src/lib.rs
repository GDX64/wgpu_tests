use piet::{Color, FontFamily, RenderContext, Text, TextLayoutBuilder};

pub struct App<V: Component> {
    comp: V,
}

impl<V: Component> App<V> {
    pub fn draw(&mut self, piet_context: &mut V::Context) {
        self.comp.update(piet_context);
        self.comp.draw(piet_context);
        piet_context.finish().unwrap();
    }
}

pub fn create_new_app<C: RenderContext>() -> App<impl Component<Context = C>> {
    App {
        comp: Button::<C>::new("Hello, world!".to_string()),
    }
}

pub trait Component {
    type Context: RenderContext;
    fn draw(&self, piet_context: &mut Self::Context);
    fn update(&mut self, piet_context: &mut Self::Context) {}
}

struct Button<C: RenderContext> {
    text: String,
    text_layout: Option<C::TextLayout>,
}

impl<C: RenderContext> Button<C> {
    fn new(text: String) -> Self {
        Self {
            text,
            text_layout: None,
        }
    }
}

impl<C: RenderContext> Component for Button<C> {
    type Context = C;
    fn draw(&self, piet_context: &mut C) {
        if let Some(txt) = &self.text_layout {
            piet_context.draw_text(txt, (100., 100.));
        }
    }

    fn update(&mut self, piet_context: &mut C) {
        let txt = piet_context
            .text()
            .new_text_layout(self.text.clone())
            .font(FontFamily::SYSTEM_UI, 12.0)
            .text_color(Color::BLUE)
            .build()
            .ok();
        if let Some(txt) = txt {
            self.text_layout = Some(txt);
        }
    }
}
