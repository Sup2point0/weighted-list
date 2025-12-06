use crate::WeightedList;
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
        unimplemented!()
        // let mut current_step = self.len();
    }
}

// == INDEXING == //
