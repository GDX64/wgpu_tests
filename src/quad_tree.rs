use piet::kurbo::{Circle, Rect, Shape};

use crate::{
    particle::GeoQuery,
    v2::{TreeValue, V2},
};

pub enum QuadTreeNode<T> {
    Empty,
    Leaf { value: T },
    Node(Box<[QuadTree<T>; 4]>),
}

pub struct QuadTree<T> {
    node: QuadTreeNode<T>,
    center: V2,
    half_width: f64,
    half_height: f64,
    rect: Rect,
}

pub enum Quadrant {
    NW,
    NE,
    SW,
    SE,
}

impl<T> QuadTreeNode<T> {
    fn take(&mut self) -> QuadTreeNode<T> {
        std::mem::replace(self, QuadTreeNode::Empty)
    }
}

impl<T: TreeValue> QuadTree<T> {
    pub fn new(center: V2, half_width: f64, half_height: f64) -> QuadTree<T> {
        QuadTree {
            rect: Rect::new(
                center.x - half_width,
                center.y - half_height,
                center.x + half_width,
                center.y + half_height,
            ),
            node: QuadTreeNode::Empty,
            center,
            half_width,
            half_height,
        }
    }

    pub fn add_vec(&mut self, vec: Vec<T>) {
        vec.into_iter().for_each(|v| {
            self.insert(v);
        });
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn for_each(&self, f: &mut impl FnMut(&QuadTree<T>)) {
        match &self.node {
            QuadTreeNode::Empty => {}
            QuadTreeNode::Leaf { value: _ } => {
                f(self);
            }
            QuadTreeNode::Node(v) => {
                let nw = &v[0];
                let ne = &v[1];
                let sw = &v[2];
                let se = &v[3];
                nw.for_each(f);
                ne.for_each(f);
                sw.for_each(f);
                se.for_each(f);
            }
        }
    }

    pub fn insert(&mut self, mut value: T) {
        let node = self.node.take();
        self.node = match node {
            QuadTreeNode::Empty => QuadTreeNode::Leaf { value },
            QuadTreeNode::Leaf { value: this_value } => {
                let mut other =
                    QuadTree::new_node(self.center.clone(), self.half_width, self.half_height);
                if value.position().sub(&this_value.position()).len() < 0.0001 {
                    value.offset_pos();
                    return;
                }
                other.insert(this_value);
                other.insert(value);
                *self = other;
                return;
            }
            QuadTreeNode::Node(mut v) => {
                let quadrant = self.quadrant(&value.position());
                match quadrant {
                    Quadrant::NW => v[0].insert(value),
                    Quadrant::NE => v[1].insert(value),
                    Quadrant::SW => v[2].insert(value),
                    Quadrant::SE => v[3].insert(value),
                };
                QuadTreeNode::Node(v)
            }
        }
    }

    pub fn new_node(center: V2, half_width: f64, half_height: f64) -> QuadTree<T> {
        let nw = QuadTree::new(
            center.sub(&V2::new(half_width / 2., half_height / 2.)),
            half_width / 2.,
            half_height / 2.,
        );
        let ne = QuadTree::new(
            center.add(&V2::new(half_width / 2., -half_height / 2.)),
            half_width / 2.,
            half_height / 2.,
        );
        let sw = QuadTree::new(
            center.add(&V2::new(-half_width / 2., half_height / 2.)),
            half_width / 2.,
            half_height / 2.,
        );
        let se = QuadTree::new(
            center.add(&V2::new(half_width / 2., half_height / 2.)),
            half_width / 2.,
            half_height / 2.,
        );
        QuadTree {
            node: QuadTreeNode::Node(Box::new([nw, ne, sw, se])),
            rect: Rect::new(
                center.x - half_width,
                center.y - half_height,
                center.x + half_width,
                center.y + half_height,
            ),
            center,
            half_width,
            half_height,
        }
    }

    fn quadrant(&self, point: &V2) -> Quadrant {
        let center = &self.center;
        if point.x < center.x {
            if point.y < center.y {
                Quadrant::NW
            } else {
                Quadrant::SW
            }
        } else {
            if point.y < center.y {
                Quadrant::NE
            } else {
                Quadrant::SE
            }
        }
    }

    fn _query_distance(&self, r: &Rect, f: &mut impl FnMut(&T)) {
        let rect = self.get_rect();
        if !rects_intersect(&rect, r) {
            return;
        }
        // rect.bounding_box()
        match &self.node {
            QuadTreeNode::Empty => {}
            QuadTreeNode::Leaf { value } => f(value),
            QuadTreeNode::Node(arr) => {
                arr[0]._query_distance(r, f);
                arr[1]._query_distance(r, f);
                arr[2]._query_distance(r, f);
                arr[3]._query_distance(r, f);
            }
        }
    }
}

impl<T: TreeValue> GeoQuery<T> for QuadTree<T> {
    fn query_distance(&self, point: &V2, r: f64, mut f: impl FnMut(&T)) {
        let circ = Circle::new((point.x, point.y), r).bounding_box();
        self._query_distance(&circ, &mut f);
    }

    fn from_vec(vec: Vec<T>, max_dim: f64) -> Self {
        let mut tree = QuadTree::new(V2::new(0., 0.), max_dim, max_dim);
        tree.add_vec(vec);
        tree
    }
}

fn rects_intersect(a: &Rect, b: &Rect) -> bool {
    a.x0 < b.x1 && a.x1 > b.x0 && a.y0 < b.y1 && a.y1 > b.y0
}

#[cfg(test)]
mod tests {
    use super::*;

    impl TreeValue for V2 {
        fn position(&self) -> V2 {
            self.clone()
        }
        fn offset_pos(&mut self) {
            *self = self.add(&V2::new(0.0001, 0.0001));
        }
    }

    #[test]
    fn test_insert() {
        // let mut tree = QuadTree::new(V2::new(0., 0.), 1., 1.);
        // tree.insert(V2::new(0.5, 0.5));
        // tree.insert(V2::new(0.25, 0.25));
        // tree.insert(V2::new(0.75, 0.75));
        // tree.insert(V2::new(0.125, 0.125));

        // let v = tree.query_distance(&V2::new(0.5, 0.5), 1.0);
        // assert_eq!(v.len(), 4)
    }
}
