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
}
