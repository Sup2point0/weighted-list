use std::fmt;
use std::{iter::*, ops::*};

use bon::{bon};
use num_traits as nums;
use rand::prelude::*;


pub trait Weight:
    nums::NumAssign
    + nums::NumCast
    + Copy
    + PartialOrd
    + Sum
    + fmt::Display
{}

impl<Type> Weight for Type
    where Type:
        nums::NumAssign
        + nums::NumCast
        + Copy
        + PartialOrd
        + Sum
        + fmt::Display
{}


// == WEIGHTED ITEM == //
// --------------------------------------------------------------------- //

#[derive(Debug, Clone, Eq, PartialEq)]
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
        write!(f, "{{ weight: {}, value: {} }}", self.weight, self.value)
    }
}


// == WEIGHTED LIST == //
// --------------------------------------------------------------------- //

#[derive(Debug, Clone)]
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
    ( $( $item: expr ),+ $(,)? ) => {
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

    pub fn items(&self) -> &Vec<WeightedItem<V,W>>
    {
        &self.data
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

// == INTERNAL == //
impl<V, W: Weight> WeightedList<V,W>
{
    fn unweight_index_nopanic(&self, weighted_index: W) -> usize
    {
        let mut t = W::zero();
        let mut i = 0;

        for each in &self.data {
            t += each.weight;

            if t > weighted_index {
                return i;
            }

            i += 1;
        }

        i
    }

    fn unweight_index(&self, weighted_index: W) -> usize
    {
        let mut t = W::zero();
        let mut i = 0;

        for each in &self.data {
            t += each.weight;

            if t > weighted_index {
                return i;
            }

            i += 1;
        }

        panic!(
            "index out of bounds: the len is {} but the index is {}",
            self.len(), weighted_index
        );
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

impl<V, W: Weight> IndexMut<W> for WeightedList<V,W>
{
    fn index_mut(&mut self, weighted_index: W) -> &mut WeightedItem<V,W>
    {
        let idx = self.unweight_index(weighted_index);
        &mut self.data[idx]
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
        self.data.insert(self.unweight_index_nopanic(weighted_index), item);
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

    pub fn pop(&mut self) -> Option<WeightedItem<V,W>>
    {
        self.data.pop()
    }

    pub fn pop_if(&mut self,
        predicate: impl FnOnce(&mut WeightedItem<V,W>) -> bool
    ) -> Option<WeightedItem<V,W>>
    {
        self.data.pop_if(predicate)
    }

    pub fn clear(&mut self) -> &Self
    {
        self.data.clear();
        self
    }

    pub fn remove(&mut self, weighted_index: W) -> WeightedItem<V,W>
    {
        self.data.remove(self.unweight_index(weighted_index))
    }
}

// == SPECIALISED MUTATION == //
impl<V: Clone, W: Weight> WeightedList<V,W>
{
    pub fn take_one(&mut self, weighted_index: W) -> WeightedItem<V,W>
    {
        self.take_by(weighted_index, W::one())
    }

    /// Decrement the weight of the item at `weighted_index` by `decrement`. If its weight becomes non-positive as a result, remove the entire item. Returns a clone of the item.
    pub fn take_by(&mut self, weighted_index: W, decrement: W) -> WeightedItem<V,W>
    {
        let idx = self.unweight_index(weighted_index);
        let target = &mut self.data[idx];
        target.weight -= decrement;

        if target.weight <= W::zero() {
            self.data.remove(idx)
        }
        else {
            target.clone()
        }
    }

    pub fn take_entire(&mut self, weighted_index: W) -> WeightedItem<V,W>
    {
        self.remove(weighted_index)
    }
}

// == RANDOM SELECTION == //
impl<V, W: Weight> WeightedList<V,W>
{
    fn get_random_weighted_index<RNG>(&self, rng: &mut RNG) -> Option<W>
        where RNG: Rng + ?Sized
    {
        let len:    f64 = nums::cast::<W, f64>(self.len())?;
        let scalar: f64 = rng.random();

        let idx = (len * scalar).floor();
        let out = nums::cast::<f64, W>(idx)?;

        Some(out)
    }

    pub fn select_random_value<RNG>(&self, rng: &mut RNG) -> Option<&V>
        where RNG: Rng + ?Sized
    {
        let out = &self.select_random_item(rng)?.value;
        Some(out)
    }

    /// Select a random item from the list.
    /// 
    /// This uses `f64` for random number generation.
    pub fn select_random_item<RNG>(&self, rng: &mut RNG) -> Option<&WeightedItem<V,W>>
        where RNG: Rng + ?Sized
    {
        if self.data.is_empty() { return None }

        let idx = self.get_random_weighted_index(rng)?;
        let out = &self[idx];

        Some(out)
    }
}

impl<V: Clone, W: Weight> WeightedList<V,W>
{
    pub fn take_one_random<RNG>(&mut self, rng: &mut RNG) -> Option<WeightedItem<V,W>>
        where RNG: Rng + ?Sized
    {
        self.take_by_random(rng, W::one())
    }

    pub fn take_by_random<RNG>(&mut self, rng: &mut RNG, decrement: W) -> Option<WeightedItem<V,W>>
        where RNG: Rng + ?Sized
    {
        if self.data.is_empty() { return None }

        let idx = self.get_random_weighted_index(rng)?;
        let out = self.take_by(idx, decrement);

        Some(out)
    }

    pub fn take_random_item_entire<RNG>(&mut self, rng: &mut RNG) -> Option<WeightedItem<V,W>>
        where RNG: Rng + ?Sized
    {
        if self.data.is_empty() { return None }

        let idx = self.get_random_weighted_index(rng)?;
        let out = self.take_entire(idx);

        Some(out)
    }
}

#[bon]
impl<V: Clone + Eq, W: Weight> WeightedList<V,W>
{
    #[builder]
    pub fn select_random_items<RNG>(&self,
        count: u32,
        rng: &mut RNG,
        unique: Option<bool>,
        replace: Option<bool>,
    ) -> Vec<V>
        where RNG: Rng + ?Sized
    {
        let unique = unique.unwrap_or(false);
        let replace = replace.unwrap_or(false);

        let mut pool = self.clone();
        let mut cand: V;

        let mut i = 0;
        let mut out = Vec::with_capacity(count as usize);

        loop
        {
            i += 1;
            if i > count { break }

            if replace {
                if let Some(item) = pool.take_by_random(rng, W::one()) {
                    cand = item.value.clone();
                } else { continue }
            }
            else {
                if let Some(value) = pool.select_random_value(rng) {
                    cand = value.clone();
                } else { continue }
            }

            if unique && out.contains(&cand) {
                continue
            } else {
                out.push(cand)
            }
        }

        out
    }
}
