#![feature(collections)]
#![feature(core)]
#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::num::Int;
use std::collections::Bound::{Unbounded, Included, Excluded};
use std::fmt::Debug;
use std::iter::{FromIterator, IntoIterator};
use std::cmp::{min, max};
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone)]
struct MarkedRangeSet<T,M> {
	ranges: BTreeMap<T, BTreeSet<M>>,
}

impl<T, M> Arbitrary for MarkedRangeSet<T,M> where T: Int + Arbitrary, M: Ord + Copy + Clone + Arbitrary {
	fn arbitrary<G: Gen>(g: &mut G) -> MarkedRangeSet<T,M> {
		let inputs: Vec<(T,T,M)> = Arbitrary::arbitrary(g);
		let mut result = MarkedRangeSet::new();
		for (lo, hi, m) in inputs.iter().cloned().map(|(s1, s2, m)| (min(s1,s2), max(s1,s2), m)) {
			result.add_range(lo, hi, m);
		}
		result
	}
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
	/*
	fn union(&self, &other: &MarkedRangeSet<T,M>) -> MarkedRangeSet<T,M> {
		self.chain(other).collect();
	}
	*/
}


/*0
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

// verifies that ranges dont repeat
#[quickcheck]
fn qc_unique_ranges(rs: MarkedRangeSet<u32, bool>) -> bool {
	let mut checker = BTreeSet::new();
	for val in rs.ranges.keys() {
		if checker.contains(val) {
			return false;
		};
		checker.insert(val);
	}
	return true;
}

// verifies that added ranges are actually marked
#[quickcheck]
fn qc_range_added(adds: Vec<(u32, u32, u8)>) -> bool {
	let mut rs = MarkedRangeSet::new();
	let sorted: Vec<(u32, u32, u8)> = adds.iter().cloned().map(|(s1, s2, m)| (min(s1,s2), max(s1,s2), m)).collect();
	for (lo, hi, m) in sorted.clone() {
		rs.add_range(lo, hi, m);
	}
	
	for (lo, hi, m) in sorted {
		for (_, set) in rs.ranges.range(Included(&lo), Included(&hi)) {
			if !set.contains(&m) {
				return false;
			};
		}
	}
	return true;
}

fn main() {
	let mut r = MarkedRangeSet::new();
	r.add_range(1,3, "R1");
	r.add_range(1,3, "R2");
	r.add_range(4,6, "R3");
	println!("{:?}",r);
}

