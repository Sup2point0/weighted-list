use std::iter::Sum;

use num_traits::Num;


#[derive(Debug)]
pub struct WeightedItem<V,W: Num>
{
    pub weight: W,
    pub value: V,
}

impl<V,W: Num> WeightedItem<V,W>
{
    pub fn unit(value: V) -> WeightedItem<V,W>
    {
        Self {
            weight: W::one(),
            value: value
        }
    }

    pub fn new(value: V, weight: W) -> WeightedItem<V,W>
    {
        Self {
            weight,
            value,
        }
    }
}

impl<V: Eq, W: Num> Eq for WeightedItem<V,W> {}

impl<V: PartialEq, W: Num> PartialEq for WeightedItem<V,W>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.value == other.value && self.weight == other.weight
    }
}


pub struct WeightedList<V,W: Num>
{
    data: Vec<WeightedItem<V,W>>,
}

// == CONSTRUCTORS == //
impl<V, W: Num> WeightedList<V,W>
{
    /// Construct an empty `WeightedList`.
    pub fn empty() -> Self
    {
        Self {
            data: Vec::new()
        }
    }

    /// Construct a `WeightedList` from an iterable of (weight, value) pairs.
    pub fn from<I>(items: I) -> Self
    where I: IntoIterator<Item = (W, V)>
    {
        Self {
            data: items.into_iter().map(
                |(weight, value)|
                WeightedItem::new(value, weight)
            ).collect::<Vec<WeightedItem<V,W>>>()
        }
    }

    pub fn new(
        items: Vec<(W, V)>
    ) -> Self
    {
        Self {
            data: items.into_iter().map(
                |(weight, value)|
                WeightedItem::new(value, weight)
            ).collect::<Vec<WeightedItem<V,W>>>()
        }
    }
}

// == PROPERTIES == //
impl<V, W: Num + Sum + Copy> WeightedList<V,W>
{
    /// Sum the weights of all items in the list.
    pub fn len(&self) -> W
    {
        self.data.iter().map(|item| item.weight).sum()
    }

    /// Get an iterator over immutable references to the values of each item in the list.
    pub fn values(&self) -> impl Iterator<Item = &V>
    {
        self.data.iter().map(|item| &item.value)
    }

    /// Get an iterator over copies of the weights of each item in the list.
    pub fn weights(&self) -> impl Iterator<Item = W>
    {
        self.data.iter().map(|item| item.weight)
    }

    /// Get an iterator over (weight, value) tuples representing each item in the list.
    /// 
    /// This satisfies the axiom:
    /// 
    /// ```rs
    /// let wl: WeightedList<V,W>;
    /// WeightedList::from(wl.raw()) == wl
    /// ```
    pub fn raw(&self) -> impl Iterator<Item = (W,&V)>
    {
        self.data.iter().map(|item| (item.weight, &item.value))
    }
}

impl<V: Eq, W: Num> Eq for WeightedList<V, W> {}

impl<V: PartialEq, W: Num> PartialEq for WeightedList<V, W>
{
    fn eq(&self, other: &Self) -> bool
    {
        if self.data.len() != other.data.len() {
            return false;
        }

        let pairs = self.data.iter()
                    .zip(other.data.iter());

        for (left, right) in pairs {
            if left != right {
                return false;
            }
        }

        true
    }
}
