use std::fmt;
use std::{iter::*, ops::*};

use num_traits::Num;


pub trait Weight: Num + PartialOrd + AddAssign + Sum + Copy + fmt::Display {}

impl<Type: Num + PartialOrd + AddAssign + Sum + Copy + fmt::Display> Weight for Type {}


#[derive(Debug, Clone)]
pub struct WeightedItem<V, W: Weight>
{
    pub weight: W,
    pub value: V,
}

impl<V, W: Weight> WeightedItem<V,W>
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

impl<V: Eq, W: Weight> Eq for WeightedItem<V,W> {}

impl<V: PartialEq, W: Weight> PartialEq for WeightedItem<V,W>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.value == other.value && self.weight == other.weight
    }
}

impl<V: fmt::Display, W: Weight> fmt::Display for WeightedItem<V,W>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{{ weight: {}, value: {} }}", self.weight, self.value)
    }
}


#[derive(Debug)]
pub struct WeightedList<V, W: Weight>
{
    data: Vec<WeightedItem<V,W>>,
}

// == CONSTRUCTORS == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Construct an empty `WeightedList`.
    pub fn new() -> Self
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
}

// == PROPERTIES == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Sum the weights of all items in the list.
    pub fn len(&self) -> W
    {
        self.data.iter().map(|item| item.weight).sum()
    }

    pub fn total_values(&self) -> usize
    {
        self.data.len()
    }

    /// Get an iterator over copies of the weights of each item in the list.
    pub fn weights(&self) -> impl Iterator<Item = W>
    {
        self.data.iter().map(|item| item.weight)
    }

    /// Get an iterator over immutable references to the values of each item in the list.
    pub fn values(&self) -> impl Iterator<Item = &V>
    {
        self.data.iter().map(|item| &item.value)
    }

    /// Get an iterator over (weight, value) tuples representing each item in the list.
    /// 
    /// This satisfies the axiom:
    /// 
    /// ```rust
    /// # use weighted_list::WeightedList;
    /// let wl = WeightedList::from([(2, "sup"), (3, "nova")]);
    /// // assert_eq!(WeightedList::from(wl.raw()), wl)
    /// ```
    pub fn raw(&self) -> impl Iterator<Item = (W,&V)>
    {
        self.data.iter().map(|item| (item.weight, &item.value))
    }
}

// == EQUALITY == //
impl<V: Eq, W: Weight> Eq for WeightedList<V, W> {}

impl<V: PartialEq, W: Weight> PartialEq for WeightedList<V, W>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.data == other.data
    }
}

// == INDEXING == //
impl<V: fmt::Display, W: Weight> Index<W> for WeightedList<V,W>
{
    type Output = WeightedItem<V,W>;

    fn index(&self, weighted_index: W) -> &Self::Output
    {
        let mut t = W::zero();

        for item in &self.data {
            t += item.weight;

            if t >= weighted_index {
                return item;
            }
        };

        // panic!("index out of bounds: the len is {} but the index is {weighted_index}", self.len());
        panic!();
    }
}

// == ITERATION == //
impl<V, W: Weight> WeightedList<V,W>
{
    pub fn iter(&self) -> impl Iterator<Item = &WeightedItem<V,W>>
    {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut WeightedItem<V,W>>
    {
        self.data.iter_mut()
    }
}

impl<V, W: Weight> IntoIterator for WeightedList<V,W>
{
    type Item = WeightedItem<V,W>;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter
    {
        self.data.into_iter()
    }
}
