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

    pub fn new(weight: W, value: V) -> WeightedItem<V,W>
    {
        Self { weight, value }
    }

    pub fn from((weight, value): (W, V)) -> WeightedItem<V,W>
    {
        Self { weight, value }
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

    /// Construct an empty `WeightedList` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self
    {
        Self {
            data: Vec::with_capacity(capacity)
        }
    }

    /// Construct a `WeightedList` from an iterable of (weight, value) pairs.
    pub fn from<I>(items: I) -> Self
        where I: IntoIterator<Item = (W, V)>
    {
        Self {
            data: items.into_iter().map(
                |(weight, value)|
                WeightedItem::new(weight, value)
            ).collect::<Vec<WeightedItem<V,W>>>()
        }
    }
}

/// Construct a `WeightedList` from the provided (weight, value) tuples.
#[macro_export]
macro_rules! wlist {
    ( $( $item: expr ),* ) => {
        WeightedList::from([
            $( $item, )*
        ]);
    };
}

// == PROPERTIES == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Sum the weights of all items in the list.
    pub fn len(&self) -> W
    {
        self.data.iter().map(|item| item.weight).sum()
    }

    pub fn is_empty(&self) -> bool
    {
        self.data.is_empty()
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
impl<V, W: Weight> Index<W> for WeightedList<V,W>
{
    type Output = WeightedItem<V,W>;

    fn index(&self, weighted_index: W) -> &Self::Output
    {
        let mut t = W::zero();

        for item in &self.data {
            t += item.weight;

            if t > weighted_index {
                return item;
            }
        };

        panic!("index out of bounds: the len is {} but the index is {weighted_index}", self.len());
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

// == LIST MUTATION == //
impl<V, W: Weight> WeightedList<V,W>
{
    pub fn push_item(&mut self, item: WeightedItem<V,W>) -> &Self
    {
        self.data.push(item);
        self
    }

    pub fn push_new_item(&mut self,
        weight: W,
        value: V
    ) -> &Self
    {
        self.push_item(WeightedItem { weight, value })
    }
    
    pub fn push_value(&mut self, value: V) -> &Self
    {
        self.push_item(WeightedItem::unit(value))
    }

    /// Insert a `WeightedItem` into the list at `weighted_index`. If `weighted_index >= len`, the item is appended to the end (the function does *not* panic).
    pub fn insert_item(&mut self,
        weighted_index: W,
        item: WeightedItem<V,W>
    ) -> &Self
    {
        let mut t = W::zero();
        let mut i: usize = 0;

        for each in &self.data {
            t += each.weight;

            if t > weighted_index {
                break;
            }

            i += 1;
        }

        self.data.insert(i, item);

        self
    }

    /// Insert an item with `weight` and `value` into the list at `weighted_index`. If `weighted_index >= len`, the item is appended to the end (the function does *not* panic).
    pub fn insert_new_item(&mut self,
        weighted_index: W,
        (weight, value): (W, V)
    ) -> &Self
    {
        self.insert_item(weighted_index, WeightedItem::new(weight, value))
    }

    /// Insert an item with `value` and a weight of `1` into the list at `weighted_index`. If `weighted_index >= len`, the item is appended to the end (the function does *not* panic).
    pub fn insert_value(&mut self,
        weighted_index: W,
        value: V,
    ) -> &Self
    {
        self.insert_item(weighted_index, WeightedItem::unit(value))
    }
}

// == RANDOM SELECTION == //
use rand::prelude::*;

use duplicate::duplicate_item;

#[duplicate_item(int; [i8]; [i16]; [i32]; [i64])]
impl<V> WeightedList<V, int>
{
    pub fn select_random_value<RNG>(&self, rng: &mut RNG) -> &V
        where RNG: Rng + ?Sized
    {
        &self.select_random_item(rng).value
    }

    /// Select a random item from the list.
    /// 
    /// This uses `f64` for random number generation.
    pub fn select_random_item<RNG>(&self, rng: &mut RNG) -> &WeightedItem<V, int>
        where RNG: Rng + ?Sized
    {
        let scalar: f64 = rng.random();
        let weighted_index = scalar * self.len() as f64;
        &self[weighted_index.round() as int]
    }
}
