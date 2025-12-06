use crate::root::*;
use crate::FrozenWeightedItem;


pub struct FrozenWeightedList<V, W: Weight>
{
    data: Vec<FrozenWeightedItem<V,W>>
}

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
