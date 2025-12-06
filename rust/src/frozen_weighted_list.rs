use std::{
    fmt::{self, Display},
    iter::*,
    ops::*,
    slice,
};

use itertools::Itertools;

use crate::root::*;
use crate::FrozenWeightedItem;


pub struct FrozenWeightedList<V, W: Weight>
{
    data: Vec<FrozenWeightedItem<V,W>>
}

// == CONSTRUCTORS == //
impl<V, W: Weight> FrozenWeightedList<V,W>
{
    pub fn new() -> Self
    {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self
    {
        Self { data: Vec::with_capacity(capacity) }
    }

    pub fn init<I>(items: I) -> Self
        where I: IntoIterator<Item = (W, V)>
    {
        let mut cumulative_weight = W::zero();

        let data = items.into_iter()
            .map(|(weight, value)| {
                cumulative_weight += weight;
                FrozenWeightedItem::new(cumulative_weight, weight, value)
            })
            .collect::<Vec<FrozenWeightedItem<V,W>>>()
        ;

        Self { data }
    }
}

#[macro_export]
macro_rules! fwlist {
    ( $( $item: expr ),* $(,)? ) => {
        FrozenWeightedList::init([
            $( $item, )*
        ])
    };
}

// == ACCESSORS == //

// == PROPERTIES == //
impl<V, W: Weight> FrozenWeightedList<V,W>
{
    pub fn len(&self) -> W
    {
        self.data
            .last()
            .map(|item| item.cumulative_weight())
            .unwrap_or(W::zero())
    }

    pub fn total_values(&self) -> usize
    {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool
    {
        self.data.is_empty()
    }

    pub fn is_zero(&self) -> bool
    {
        !self.is_empty()
        && self.data.iter().all(|item| item.weight() == W::zero())
    }
}

// == CONVERSIONS == //
impl<V, W: Weight> From<Vec<FrozenWeightedItem<V,W>>> for FrozenWeightedList<V,W>
{
    fn from(vec: Vec<FrozenWeightedItem<V,W>>) -> Self {
        Self { data: vec }
    }
}

impl<V, W: Weight> Into<Vec<FrozenWeightedItem<V,W>>> for FrozenWeightedList<V,W>
{
    fn into(self) -> Vec<FrozenWeightedItem<V,W>> {
        self.data
    }
}

impl<V, W: Weight> AsRef<Vec<FrozenWeightedItem<V,W>>> for FrozenWeightedList<V,W>
{
    fn as_ref(&self) -> &Vec<FrozenWeightedItem<V,W>> {
        &self.data
    }
}

impl<V, W: Weight> Deref for FrozenWeightedList<V,W>
{
    type Target = [FrozenWeightedItem<V,W>];

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<V, W: Weight> DerefMut for FrozenWeightedList<V,W>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}

// == TRAITS == //
impl<V: Display, W: Weight + Display> Display for FrozenWeightedList<V,W>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {            
        write!(f,
            "FrozenWeightedList[{}]",
            self.data.iter().map(|item| item.to_string()).join(", ")
        )
    }
}

// == INTERNAL == //
impl<V, W: Weight> FrozenWeightedList<V,W>
{
    fn _binary_unweight_index_(&self, weighted_index: W) -> usize
    {
        let max = self.total_values();
        let mut left_idx:  usize = 0;
        let mut right_idx: usize = max - 1;

        for _ in 0..(max / 2) {
            let pivot_idx = left_idx.midpoint(right_idx);
            let cand = &self.data[pivot_idx];
            let weight = cand.weight();
            let c_weight = cand.c_weight();

            if c_weight > weighted_index && weighted_index >= c_weight - weight {
                return pivot_idx;
            }

            if weighted_index < c_weight {
                right_idx = pivot_idx - 1;
            } else {
                left_idx = pivot_idx + 1;
            }
        }

        panic!(
            "index out of bounds: the len is {} but the index is {}",
            self.len(), weighted_index
        );
    }
}

// == INDEXING == //
impl<V, W: Weight> Index<W> for FrozenWeightedList<V,W>
{
    type Output = FrozenWeightedItem<V,W>;

    fn index(&self, weighted_index: W) -> &Self::Output {
        &self.data[self._binary_unweight_index_(weighted_index)]
    }
}

impl<V, W: Weight> IndexMut<W> for FrozenWeightedList<V,W>
{
    fn index_mut(&mut self, weighted_index: W) -> &mut Self::Output
    {
        let idx = self._binary_unweight_index_(weighted_index);
        &mut self.data[idx]
    }
}

impl<V, W: Weight> FrozenWeightedList<V,W>
{
    pub fn get(&self, weighted_index: W) -> Option<&FrozenWeightedItem<V,W>>
    {
        self.data.get(self._binary_unweight_index_(weighted_index))
    }
}

// == ITERATION == //
impl<V, W: Weight> IntoIterator for FrozenWeightedList<V,W>
{
    type Item = FrozenWeightedItem<V,W>;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'l, V, W: Weight> IntoIterator for &'l FrozenWeightedList<V,W>
{
    type Item = &'l FrozenWeightedItem<V,W>;
    type IntoIter = slice::Iter<'l, FrozenWeightedItem<V,W>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'l, V, W: Weight> IntoIterator for &'l mut FrozenWeightedList<V,W>
{
    type Item = &'l mut FrozenWeightedItem<V,W>;
    type IntoIter = slice::IterMut<'l, FrozenWeightedItem<V,W>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}


// == INTERNAL TESTS == //

#[cfg(test)]
mod tests
{
    use super::*;

    fn fwl() -> FrozenWeightedList<String, i32>
    {
        fwlist![
            (2, String::from("sup")),
            (3, String::from("nova")),
            (5, String::from("shard")),
        ]
    }

    #[test]
    fn _unweight_index_()
    {
        let list = fwl();
        assert_eq!( list._binary_unweight_index_(0), 0 );
        assert_eq!( list._binary_unweight_index_(1), 0 );
        assert_eq!( list._binary_unweight_index_(2), 1 );
        assert_eq!( list._binary_unweight_index_(3), 1 );
        assert_eq!( list._binary_unweight_index_(4), 1 );
        assert_eq!( list._binary_unweight_index_(5), 2 );
        assert_eq!( list._binary_unweight_index_(6), 2 );
        assert_eq!( list._binary_unweight_index_(7), 2 );
        assert_eq!( list._binary_unweight_index_(8), 2 );
        assert_eq!( list._binary_unweight_index_(9), 2 );
    }
}
