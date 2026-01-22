use std::{
    fmt
};

use crate::root::*;


/// A shorthand for [`WeightedItem`].
/// 
/// If you refer to [`WeightedItem`] prolifically in your code, you may wish to use this for brevity. Otherwise, the full [`WeightedItem`] is recommended for clarity.
pub type WItem<V,W> = WeightedItem<V,W>;


/// An item in a [`WeightedList`], with a `value` of type `V` and a `weight` of numerical type `W`.
/// 
/// For consistency and layout, `weight` always comes before `value` when ordering is relevant.
/// 
/// You should rarely find yourself constructing a [`WeightedItem`] by hand – instead, you'll usually interact with existing instances from a [`WeightedList`](crate::WeightedList).
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct WeightedItem<V, W: Weight>
{
    /// The weight of the item. A positive number. `0` is technically valid, but not advised.
    /// 
    /// > [!Note]
    /// > `num_traits::Unsigned` is not enforced because this is incompatible with non-integer `W` (`f32`, `f64`, etc.) which are always signed.
    pub weight: W,

    /// The value stored in the item.
    pub value: V,
}

// == CONSTRUCTORS == //
impl<V, W: Weight> WeightedItem<V,W>
{
    /// Construct an item with `value` and a weight of `1`.
    pub fn unit(value: V) -> Self
    {
        Self {
            weight: W::one(),
            value
        }
    }

    /// Construct an item with `value` and `weight`.
    pub fn new(weight: W, value: V) -> Self
    {
        Self { weight, value }
    }

    /// Construct an item from a `(weight, value)` pair.
    pub fn from((weight, value): (W, V)) -> Self
    {
        Self { weight, value }
    }
}

/// Construct a [`WeightedItem`] from a `(weight, value)` pair.
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
    ($weight: expr, $value: expr) => {
        WeightedItem::new($weight, $value)
    };
}

// == CONVERSIONS == //
impl<V, W: Weight> From<WeightedItem<V,W>> for (W, V)
{
    fn from(item: WeightedItem<V,W>) -> Self {
        (item.weight, item.value)
    }
}

// == TRAIT IMPLEMENTATIONS == //
impl<V, W: Weight> Ord for WeightedItem<V,W>
    where
        V: Eq,
        W: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.weight.cmp(&other.weight)
            // .then(self.value.cmp(&other.value))  // TODO FIXME
    }
}

impl<V, W: Weight> PartialOrd for WeightedItem<V,W>
    where
        V: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<V, W: Weight> Default for WeightedItem<V,W>
    where
        V: Default,
        W: Default,
{
    fn default() -> Self {
        Self { weight: W::default(), value: V::default() }
    }
}

impl<V, W: Weight> fmt::Display for WeightedItem<V,W>
    where
        V: fmt::Display,
        W: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{{ {}, {} }}", self.weight, self.value)
    }
}
