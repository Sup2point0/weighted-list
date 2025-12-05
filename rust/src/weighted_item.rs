use std::{
    fmt,
};

use crate::root::*;


/// An item in a `WeightedList`, with a `value` of type `V` and a `weight` of numerical type `W`.
/// 
/// For consistency and layout, `weight` always comes before `value` when ordering is relevant.
/// 
/// You should rarely find yourself constructing a `WeightedItem` by hand – instead, you'll usually interact with existing instances from a `WeightedList`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WeightedItem<V, W: Weight>
{
    /// The weight of the item. A non-negative number. `0` is technically valid, but not advised.
    pub weight: W,

    /// The value stored in the item.
    pub value: V,
}

impl<V, W: Weight> WeightedItem<V,W>
{
    /// Construct a `WeightedItem` with `value` and a weight of `1`.
    pub fn unit(value: V) -> Self
    {
        Self {
            weight: W::one(),
            value: value
        }
    }

    /// Construct a `WeightedItem` with `value` and `weight`.
    pub fn new(weight: W, value: V) -> Self
    {
        Self { weight, value }
    }

    /// Construct a `WeightedItem` from a `(weight, value)` pair.
    pub fn from((weight, value): (W, V)) -> Self
    {
        Self { weight, value }
    }
}

/// Construct a `WeightedItem` from a `(weight, value)`` pair.
/// 
/// # Usage
/// 
/// ```
/// # use weighted_list::*;
/// let item = wit!(2.0, "sup");
/// assert_eq!(item, WeightedItem::new(2.0, "sup"));
/// ```
#[macro_export]
macro_rules! wit {
    ( $weight: expr, $value: expr ) => {
        WeightedItem::new($weight, $value)
    };
}

impl<V: fmt::Display, W: Weight> fmt::Display for WeightedItem<V,W>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{{ {}, {} }}", self.weight, self.value)
    }
}

impl<V: Eq, W: Weight + Ord> Ord for WeightedItem<V,W>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.weight.cmp(&other.weight)
    }
}

impl<V: Eq, W: Weight> PartialOrd for WeightedItem<V,W>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.weight.partial_cmp(&other.weight)
    }
}
