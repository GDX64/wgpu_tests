use piet::kurbo::{Circle, Rect, Shape};

use crate::{particle::GeoQuery, quad_tree::TreeValue, v2::V2};

struct OrderStore<T> {
    value: T,
    order: usize,
}

pub struct ZOrderTree<T> {
    values: Vec<OrderStore<T>>,
    order: usize,
    max_order_value: f64,
    max_dim: f64,
}

impl<T: TreeValue> ZOrderTree<T> {
    pub fn from_vec(vec: Vec<T>, order: usize, max_dim: f64) -> ZOrderTree<T> {
        let mut tree = ZOrderTree {
            values: Vec::new(),
            order,
            max_dim,
            max_order_value: (1 << order) as f64,
        };
        let mut v = vec
            .into_iter()
            .map(|value| {
                let order = tree.order_of(value.position().x, value.position().y);
                OrderStore { value, order }
            })
            .collect::<Vec<OrderStore<T>>>();
        v.sort_by_key(|v| v.order);
        tree.values = v;
        tree
    }

    fn query_rect(&self, rect: &Rect) -> &[OrderStore<T>] {
        let start_order = self.order_of(rect.x0, rect.y0);
        let start = self.find_order_index(start_order);
        let end_order = self.order_of(rect.x1, rect.y1);
        let end = self.find_order_index(end_order);
        &self.values[start..end]
    }

    fn find_order_index(&self, order: usize) -> usize {
        let r = self.values.binary_search_by(|value| {
            if value.order == order {
                return std::cmp::Ordering::Equal;
            }
            if value.order < order {
                return std::cmp::Ordering::Less;
            }
            return std::cmp::Ordering::Greater;
        });
        match r {
            Ok(i) => i,
            Err(i) => i,
        }
    }

    fn order_of(&self, x: f64, y: f64) -> usize {
        let x = (x / self.max_dim * self.max_order_value) as usize;
        let y = (y / self.max_dim * self.max_order_value) as usize;
        z_order(x, y, self.order)
    }
}

fn z_order(x: usize, y: usize, order: usize) -> usize {
    let mut z = 0;
    for i in 0..order {
        z |= ((x & (1 << i)) << i) | ((y & (1 << i)) << (i + 1));
    }
    z
}

impl<T: TreeValue> GeoQuery<T> for ZOrderTree<T> {
    fn query_distance(&self, point: &V2, radius: f64, mut f: impl FnMut(&T)) {
        let rect = Circle::new((point.x, point.y), radius).bounding_box();
        self.query_rect(&rect)
            .iter()
            .for_each(|value| f(&value.value));
    }

    fn from_vec(vec: Vec<T>, max_dim: f64) -> Self {
        ZOrderTree::from_vec(vec, 32, max_dim)
    }
}

#[cfg(test)]
mod test {
    use crate::zorder_tree::z_order;

    #[test]
    fn test_order() {
        assert!(z_order(0, 0, 10) == 0);
        assert!(z_order(1, 0, 10) == 1);
        assert!(z_order(0, 1, 10) == 2);
        assert!(z_order(1, 1, 10) == 3);
        assert!(z_order(2, 0, 10) == 4);
        assert!(z_order(2, 2, 10) == 12);
        assert!(z_order(7, 7, 10) == 63);
    }
}
