use std::ops::Index;

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

// == INTERNAL == //
impl<V, W: Weight> FrozenWeightedList<V,W>
{
    fn _binary_unweight_index_(&self, weighted_index: W) -> usize
    {
        let max = self.total_values();

        let mut left_idx:  usize = 0;
        let mut right_idx: usize = max - 1;
        let mut pivot_idx: usize;

        let mut cand: &FrozenWeightedItem<V,W>;
        let mut weight: W;
        let mut c_weight: W;

        for _ in 0..max {
            pivot_idx = left_idx.midpoint(right_idx);

            cand = &self.data[pivot_idx];
            weight = cand.weight();
            c_weight = cand.c_weight();

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
