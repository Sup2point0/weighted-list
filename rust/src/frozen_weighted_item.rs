use crate::root::*;


pub struct FrozenWeightedItem<V, W: Weight>
{
    cumulative_weight: Option<W>,
    pub weight: W,
    pub value: V,
}

impl<V, W: Weight> FrozenWeightedItem<V,W>
{
    pub fn unit(value: V) -> Self
    {
        Self {
            cumulative_weight: None,
            weight: W::one(),
            value,
        }
    }

    pub fn new(weight: W, value: V) -> Self
    {
        Self {
            cumulative_weight: None,
            weight,
            value,
        }
    }

    pub fn from((weight, value): (W, V)) -> Self
    {
        Self {
            cumulative_weight: None,
            weight,
            value,
        }
    }
}
