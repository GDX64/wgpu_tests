//! Basic example of rendering on Direct2D.
use std::{error::Error, time::Duration};

use piet::{
    kurbo, Color, FontFamily, ImageBuf, RenderContext, Text, TextLayout, TextLayoutBuilder,
};

pub fn run_sample(piet_context: &mut impl RenderContext) -> Result<(), Box<dyn Error>> {
    let shape = kurbo::Circle::new((50., 50.), 40.);

    let txt = piet_context
        .text()
        .new_text_layout("hello")
        .font(FontFamily::SYSTEM_UI, 12.0)
        .text_color(Color::BLUE)
        .build()?;
    piet_context.draw_text(&txt, (100., 100.));
    // We need to postpone returning a potential error to ensure cleanup
    piet_context.fill(shape, &Color::YELLOW);
    piet_context.finish()?;
    Ok(())
}
