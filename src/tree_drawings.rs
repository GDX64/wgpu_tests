use piet::{kurbo::Circle, Color};

use crate::{
    particle::{GeoQuery, PARTICLE_RADIUS},
    quad_tree::{QuadTree, TreeValue},
    v2::V2,
    zorder_tree::ZOrderTree,
};

pub trait TreeDrawable {
    fn draw(&self, piet_context: &mut impl piet::RenderContext, mouse_pos: &V2) -> Option<()>;
}

impl<T: TreeValue> TreeDrawable for QuadTree<T> {
    fn draw(&self, piet_context: &mut impl piet::RenderContext, mouse_pos: &V2) -> Option<()> {
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

impl<T: TreeValue> TreeDrawable for ZOrderTree<T> {
    fn draw(&self, piet_context: &mut impl piet::RenderContext, mouse_pos: &V2) -> Option<()> {
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
