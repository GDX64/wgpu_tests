use piet::kurbo::{Circle, Rect, Shape};
use rstar::RTreeObject;

use crate::{
    particle::GeoQuery,
    v2::{TreeValue, V2},
};

struct OrderStore<T> {
    value: T,
    order: u64,
}

pub struct RStartree<T: TreeValue> {
    tree: rstar::RTree<MyObj<T>>,
}

impl<T: TreeValue> RStartree<T> {
    pub fn from_vec(vec: Vec<T>, order: u64, max_dim: f64) -> Self {
        let objs = vec.into_iter().map(|value| MyObj { value }).collect();
        let star = rstar::RTree::bulk_load(objs);
        RStartree { tree: star }
    }

    // pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a T> {
    //     self.tree.iter().map(|v| &v.value)
    // }

    pub fn boundings<'a>(&'a self) -> impl Iterator<Item = Rect> + 'a {
        let root = self.tree.root();
        let result = root
            .children()
            .iter()
            .flat_map(|child| Self::traverse(child));
        result
    }

    fn traverse(node: &rstar::RTreeNode<MyObj<T>>) -> impl Iterator<Item = Rect> {
        match node {
            rstar::RTreeNode::Leaf(_) => vec![].into_iter(),
            rstar::RTreeNode::Parent(inner) => {
                let mut rects = Vec::new();
                let v = inner.envelope();
                let lower = v.lower();
                let upper = v.upper();
                let rect = Rect::new(lower[0], lower[1], upper[0], upper[1]);
                rects.push(rect);
                for child in inner.children() {
                    rects.extend(Self::traverse(child));
                }
                rects.into_iter()
            }
        }
    }
}

fn z_order(x: u64, y: u64, order: u64) -> u64 {
    let mut z = 0;
    for i in 0..order {
        z |= ((x & (1 << i)) << i) | ((y & (1 << i)) << (i + 1));
    }
    z
}

impl<T: TreeValue> GeoQuery<T> for RStartree<T> {
    fn query_distance(&self, point: &V2, radius: f64, mut f: impl FnMut(&T)) {
        let rect = Circle::new((point.x, point.y), radius).bounding_box();
        let envelope = rstar::AABB::from_corners([rect.x0, rect.y0], [rect.x1, rect.y1]);
        let slice = self.tree.locate_in_envelope(&envelope);
        slice.for_each(|value| f(&value.value));
    }

    fn from_vec(vec: Vec<T>, max_dim: f64) -> Self {
        RStartree::from_vec(vec, 32, max_dim)
    }
}

struct MyObj<T> {
    value: T,
}

impl<T: TreeValue> RTreeObject for MyObj<T> {
    type Envelope = rstar::AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let v = self.value.position();
        rstar::AABB::from_point([v.x, v.y])
    }
}

#[cfg(test)]
mod test {}
