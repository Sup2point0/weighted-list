use std::{
    fmt,
};

use crate::*;


#[derive(Debug, Clone, Hash)]
pub struct FrozenWeightedItem<V, W: Weight>
{
    cumulative_weight: W,
    weight: W,
    value: V,
}

// == CONSTRUCTORS == //
impl<V, W: Weight> FrozenWeightedItem<V,W>
{
    pub fn new(
        cumulative_weight: W,
        weight: W,
        value: V
    ) -> Self
    {
        Self { cumulative_weight, weight, value }
    }
}

/// Construct a `FrozenWeightedItem` from a `(weight, value)`` pair.
/// 
/// # Usage
/// 
/// ```
/// # use weighted_list::*;
/// let item = fwit!(2.0, "sup");
/// assert_eq!(item, FrozenWeightedItem::new(2.0, 2.0, "sup"));
/// 
/// let item = fwit!(3.0, "nova");
/// assert_eq!(item, FrozenWeightedItem::new(3.0, 3.0, "nova"));
/// ```
#[macro_export]
macro_rules! fwit {
    ( $weight: expr, $value: expr ) => {
        FrozenWeightedItem::new($weight, $weight, $value)
    };
    ( $cweight: expr, $weight: expr, $value: expr ) => {
        FrozenWeightedItem::new($cweight, $weight, $value)
    };
}

// == ACCESSORS == //
impl<V, W: Weight> FrozenWeightedItem<V,W>
{
    #[allow(dead_code)]
    pub(crate) fn c_weight(&self) -> W {
        self.cumulative_weight
    }

    pub fn cumulative_weight(&self) -> W {
        self.cumulative_weight
    }
    
    pub fn weight(&self) -> W {
        self.weight
    }
    
    pub fn value(&self) -> &V {
        &self.value
    }
}

// == CONVERSIONS == //
impl<V, W: Weight> Into<WeightedItem<V,W>> for FrozenWeightedItem<V,W>
{
    fn into(self) -> WeightedItem<V,W> {
        WeightedItem::new(self.weight, self.value)
    }
}

// == TRAITS == //
impl<V: fmt::Display, W: Weight> fmt::Display for FrozenWeightedItem<V,W>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{{ {}, {} }}", self.weight, self.value)
    }
}

impl<V: Eq, W: Weight> Eq for FrozenWeightedItem<V,W> {}

impl<V: PartialEq, W: Weight> PartialEq for FrozenWeightedItem<V,W>
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
        && self.weight == other.weight
    }
}

impl<V: Eq, W: Weight + Ord> Ord for FrozenWeightedItem<V,W>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.weight.cmp(&other.weight)
    }
}

impl<V: Eq, W: Weight> PartialOrd for FrozenWeightedItem<V,W>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.weight.partial_cmp(&other.weight)
    }
}
