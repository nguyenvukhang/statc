use crate::display::LineList;
use crate::distributions::{Continuous, Distribution};
use crate::math::Round;
use crate::Area;

pub trait Invert {
    fn invert(&self, area: Area, p: f64) -> LineList;
}

impl<D: Distribution + Continuous> Invert for D {
    fn invert(&self, area: Area, p: f64) -> LineList {
        let mut list = LineList::new();
        list.set_title(&self.title());
        match area {
            Area::Left => {
                let x = self.inv_cdf(p);
                list.push("x: right bound", x.roundn(10));
                list.push("P(X > x)", p);
            }
            Area::Right => {
                let x = self.inv_cdf(1.0 - p);
                list.push("x: left bound", x.roundn(10));
                list.push("P(X > x)", p);
            }
            Area::Mid => {
                let a = (1.0 - p) / 2.0;
                list.push("a: left bound", self.inv_cdf(a));
                list.push("b: right bound", self.inv_cdf(1.0 - a));
                list.push("P(a < X < b)", p);
            }
        }
        list
    }
}
