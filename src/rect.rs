use ordered_float::OrderedFloat;

use point::Point;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct Rect {
    pub left: OrderedFloat<f64>,
    pub top: OrderedFloat<f64>,
    pub right: OrderedFloat<f64>,
    pub bottom: OrderedFloat<f64>,
}

impl Rect {
    #[inline]
    pub fn contains_point(&self, point: Point) -> bool {
        point.x > self.left && point.y > self.top && point.x < self.right && point.y < self.bottom
    }
}
