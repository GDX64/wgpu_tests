use piet::{
    kurbo::{Affine, Circle},
    Color,
};

use crate::{
    particle::{GeoQuery, Particle, World, PARTICLE_RADIUS},
    quad_tree::QuadTree,
    rstar_tree::RStartree,
    v2::{TreeValue, V2},
    zorder_tree::ZOrderTree,
};

pub struct DrawContext {
    pub mouse_pos: Option<V2>,
}

pub trait Drawable {
    fn draw(
        &self,
        piet_context: &mut impl piet::RenderContext,
        draw_context: &DrawContext,
    ) -> Option<()>;
}

impl<T: TreeValue> Drawable for QuadTree<T> {
    fn draw(
        &self,
        piet_context: &mut impl piet::RenderContext,
        draw_context: &DrawContext,
    ) -> Option<()> {
        let mouse_pos = draw_context.mouse_pos.as_ref()?;
        let brush = Color::from_rgba32_u32(0xffff0055);
        self.for_each(&mut |n| {
            let rect = n.get_rect();
            piet_context.stroke(rect, &brush, 1.0);
        });
        let query_circ = Circle::new((mouse_pos.x, mouse_pos.y), PARTICLE_RADIUS);
        piet_context.stroke(query_circ, &Color::RED.with_alpha(0.5), 1.0);
        self.query_distance(
            &V2::new(query_circ.center.x, query_circ.center.y),
            query_circ.radius,
            |p| {
                let rect = Circle::new((p.position().x, p.position().y), 1.);
                piet_context.fill(rect, &Color::RED.with_alpha(0.8));
            },
        );
        Some(())
    }
}

impl<T: TreeValue> Drawable for ZOrderTree<T> {
    fn draw(
        &self,
        piet_context: &mut impl piet::RenderContext,
        draw_context: &DrawContext,
    ) -> Option<()> {
        let mouse_pos = draw_context.mouse_pos.as_ref()?;
        let brush = Color::from_rgba32_u32(0xffff0055);
        let mut values = self.values();
        if let Some(first) = values.next() {
            let mut line = piet::kurbo::BezPath::new();
            line.move_to((first.position().x, first.position().y));
            values.for_each(|v| {
                line.line_to((v.position().x, v.position().y));
            });
            piet_context.stroke(line, &brush, 1.0);
        } else {
            return None;
        }

        let query_circ = Circle::new((mouse_pos.x, mouse_pos.y), PARTICLE_RADIUS);
        piet_context.stroke(query_circ, &Color::RED.with_alpha(0.5), 1.0);
        self.query_distance(
            &V2::new(query_circ.center.x, query_circ.center.y),
            query_circ.radius,
            |p| {
                let rect = Circle::new((p.position().x, p.position().y), 1.);
                piet_context.fill(rect, &Color::RED.with_alpha(0.8));
            },
        );
        Some(())
    }
}

impl<T: TreeValue> Drawable for RStartree<T> {
    fn draw(
        &self,
        piet_context: &mut impl piet::RenderContext,
        draw_context: &DrawContext,
    ) -> Option<()> {
        let mouse_pos = draw_context.mouse_pos.as_ref()?;
        let brush = Color::from_rgba32_u32(0xffff0055);
        let values = self.boundings();
        values.for_each(|rect| piet_context.stroke(rect, &brush, 1.0));

        let query_circ = Circle::new((mouse_pos.x, mouse_pos.y), PARTICLE_RADIUS);
        piet_context.stroke(query_circ, &Color::RED.with_alpha(0.5), 1.0);
        self.query_distance(
            &V2::new(query_circ.center.x, query_circ.center.y),
            query_circ.radius,
            |p| {
                let rect = Circle::new((p.position().x, p.position().y), 1.);
                piet_context.fill(rect, &Color::RED.with_alpha(0.8));
            },
        );
        Some(())
    }
}

impl<T: GeoQuery<Particle> + Drawable> Drawable for World<T> {
    fn draw(
        &self,
        piet_context: &mut impl piet::RenderContext,
        draw_context: &DrawContext,
    ) -> Option<()> {
        let brush = Color::WHITE;
        self.particles.iter().for_each(|particle| {
            let x = particle.position.x;
            let y = particle.position.y;
            let v = particle.velocity.len();
            let b = brush.with_alpha((v / 50.).min(1.).max(0.3));
            let particle = Circle::new((x, y), 2.);
            piet_context.fill(particle, &b);
        });
        // let center = V2::new(WIDTH as f64 / 2., HEIGHT as f64 / 2.);
        // let gradient = self.calc_gradient(&center);
        // draw_arrow(&center, &center.add(&gradient), piet_context);
        if let Some(ref mouse_pos) = self.mouse_pos {
            if self.show_quad_tree {
                self.tree.draw(piet_context, draw_context);
            }
        };
        Some(())
    }
}
