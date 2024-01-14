use piet::kurbo::Rect;

use crate::particle::V2;

pub enum QuadTreeNode<T> {
    Empty,
    Leaf {
        value: T,
    },
    Node {
        nw: Box<QuadTree<T>>,
        ne: Box<QuadTree<T>>,
        sw: Box<QuadTree<T>>,
        se: Box<QuadTree<T>>,
    },
}

pub struct QuadTree<T> {
    node: QuadTreeNode<T>,
    center: V2,
    half_width: f64,
    half_height: f64,
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

pub trait TreeValue {
    fn position(&self) -> V2;
}

impl<T: TreeValue> QuadTree<T> {
    pub fn new(center: V2, half_width: f64, half_height: f64) -> QuadTree<T> {
        QuadTree {
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
        Rect::new(
            self.center.x - self.half_width,
            self.center.y - self.half_height,
            self.center.x + self.half_width,
            self.center.y + self.half_height,
        )
    }

    pub fn for_each(&self, f: &mut impl FnMut(&QuadTree<T>)) {
        match &self.node {
            QuadTreeNode::Empty => {}
            QuadTreeNode::Leaf { value } => {
                f(self);
            }
            QuadTreeNode::Node { nw, ne, sw, se } => {
                nw.for_each(f);
                ne.for_each(f);
                sw.for_each(f);
                se.for_each(f);
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        let node = self.node.take();
        self.node = match node {
            QuadTreeNode::Empty => QuadTreeNode::Leaf { value },
            QuadTreeNode::Leaf { value: this_value } => {
                let mut other =
                    QuadTree::new_node(self.center.clone(), self.half_width, self.half_height);
                other.insert(this_value);
                other.insert(value);
                *self = other;
                return;
            }
            QuadTreeNode::Node {
                mut nw,
                mut ne,
                mut sw,
                mut se,
            } => {
                let quadrant = self.quadrant(&value.position());
                match quadrant {
                    Quadrant::NW => nw.insert(value),
                    Quadrant::NE => ne.insert(value),
                    Quadrant::SW => sw.insert(value),
                    Quadrant::SE => se.insert(value),
                };
                QuadTreeNode::Node { nw, ne, sw, se }
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
            node: QuadTreeNode::Node {
                nw: Box::new(nw),
                ne: Box::new(ne),
                sw: Box::new(sw),
                se: Box::new(se),
            },
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

    pub fn query_distance<'a>(&'a self, point: &V2, r: f64) -> Vec<&'a T> {
        if !rect_intersect_circle(
            &Rect::new(
                self.center.x - self.half_width,
                self.center.y - self.half_height,
                self.center.x + self.half_width,
                self.center.y + self.half_height,
            ),
            point,
            r,
        ) {
            return Vec::new();
        }
        let mut result = Vec::new();
        match &self.node {
            QuadTreeNode::Empty => {}
            QuadTreeNode::Leaf { value } => {
                result.push(value);
            }
            QuadTreeNode::Node { nw, ne, sw, se } => {
                result.extend(nw.query_distance(point, r));
                result.extend(ne.query_distance(point, r));
                result.extend(sw.query_distance(point, r));
                result.extend(se.query_distance(point, r));
            }
        }
        result
    }
}

fn rect_intersect_circle(rect: &Rect, center: &V2, r: f64) -> bool {
    let mut closest_x = center.x;
    let mut closest_y = center.y;
    if center.x < rect.x0 {
        closest_x = rect.x0;
    } else if center.x > rect.x1 {
        closest_x = rect.x1;
    }
    if center.y < rect.y0 {
        closest_y = rect.y0;
    } else if center.y > rect.y1 {
        closest_y = rect.y1;
    }
    let dist_x = center.x - closest_x;
    let dist_y = center.y - closest_y;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
    distance < r
}

#[cfg(test)]
mod tests {
    use super::*;

    impl TreeValue for V2 {
        fn position(&self) -> V2 {
            self.clone()
        }
    }

    #[test]
    fn test_insert() {
        let mut tree = QuadTree::new(V2::new(0., 0.), 1., 1.);
        tree.insert(V2::new(0.5, 0.5));
        tree.insert(V2::new(0.25, 0.25));
        tree.insert(V2::new(0.75, 0.75));
        tree.insert(V2::new(0.125, 0.125));

        let v = tree.query_distance(&V2::new(0.5, 0.5), 1.0);
        assert_eq!(v.len(), 4)
    }
}
