use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::num::Int;
use std::collections::Bound::{Unbounded, Included, Excluded};
use std::fmt::Debug;
use std::iter::{FromIterator, IntoIterator};

#[derive(Debug, Clone)]
struct MarkedRangeSet<T,M> {
	ranges: BTreeMap<T, BTreeSet<M>>,
}

impl <T,M> MarkedRangeSet<T,M> where T: Int, M: Ord + Copy + Clone {
	pub fn new() -> MarkedRangeSet<T,M> {
		let mut set = MarkedRangeSet { ranges: BTreeMap::new() };
		set.ranges.insert(<T as Int>::min_value(), BTreeSet::new());
		set
	}
	
	pub fn add_range(&mut self, lo: T, hi: T, mark: M) {
		{
			let mut cut = |location: T| {
				if !self.ranges.contains_key(&location) {
					let marks = {
						let (_, prev) = self.ranges.range(Unbounded, Excluded(&location)).next_back()
								                .expect("MarkedRangeSet somehow lost its initial value!");
						prev.clone()
					};
					self.ranges.insert(location, marks);
				}
			};
			cut(lo);
			cut(hi+<T as Int>::one());
		}
		
		for (_, mset) in self.ranges.range_mut(Included(&lo), Included(&hi)) {
			mset.insert(mark.clone());
		}
	}
}

#[test]
fn combine_range() {
}

/*
impl<T,M> FromIterator<((T,T),M)> for MarkedRangeSet<T,M> where T: Int, M: Ord + Copy + Clone {
	fn from_iter<I>(iterator: I) -> Self where I: IntoIterator {
		let mut created = MarkedRangeSet::new();
		for ((lo, hi), mark) in iterator {
			created.add_range(lo, hi, mark);
		}
		created
	}
}
*/
fn main() {
	let mut r = MarkedRangeSet::new();
	r.add_range(1,3, "R1");
	r.add_range(1,3, "R2");
	r.add_range(4,6, "R3");
}

