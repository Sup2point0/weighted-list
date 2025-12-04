use std::{
    fmt,
    iter::*,
    ops::*,
};

use bon::{bon};
use num_traits as nums;
use rand::{
    prelude::*,
    seq::SliceRandom,
};


pub trait Weight:
    nums::NumAssign
    + nums::NumCast
    + Copy
    + PartialOrd
    + Sum
    + fmt::Display
{}

impl<Type> Weight for Type where Type:
    nums::NumAssign
    + nums::NumCast
    + Copy
    + PartialOrd
    + Sum
    + fmt::Display
{}


// == WEIGHTED ITEM == //
// --------------------------------------------------------------------- //

/// An item in a `WeightedList`, with a `value` of type `V` and a `weight` of numerical type `W`.
/// 
/// For consistency and layout, `weight` always comes before `value` when ordering is relevant.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeightedItem<V, W: Weight>
{
    pub weight: W,
    pub value: V,
}

impl<V, W: Weight> WeightedItem<V,W>
{
    /// Construct a `WeightedItem` with `value` and a weight of 1.
    pub fn unit(value: V) -> WeightedItem<V,W>
    {
        Self {
            weight: W::one(),
            value: value
        }
    }

    /// Construct a `WeightedItem` with `value` and `weight`.
    pub fn new(weight: W, value: V) -> WeightedItem<V,W>
    {
        Self { weight, value }
    }

    /// Construct a `WeightedItem` from a `(weight, value)` pair.
    pub fn from((weight, value): (W, V)) -> WeightedItem<V,W>
    {
        Self { weight, value }
    }
}

/// Construct a `WeightedItem` from a `(weight, value) pair`.
/// 
/// ### Usage
/// ```
/// # use weighted_list::*;
/// 
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
        write!(f, "{{ weight: {}, value: {} }}", self.weight, self.value)
    }
}

// impl<V: PartialEq, W: Weight> PartialEq for WeightedItem<V,W>
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.value == other.value && self.weight == other.weight
//     }
// }

// impl<V: Eq, W: Weight> Eq for WeightedItem<V,W> {}

impl<V: Eq, W: Weight + Ord> Ord for WeightedItem<V,W>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<V: Eq, W: Weight> PartialOrd for WeightedItem<V,W>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}


// == WEIGHTED LIST == //
// --------------------------------------------------------------------- //

/// A homogeneous list of weighted items with values of type `V` and weights of numerical type `W`.
/// 
/// Near-identical to `Vec<T>`, but stores `WeightedItem<V,W>` objects instead. You can think of it like a `Vec<WeightedItem<V,W>>`.
/// 
/// ### Usage
/// ```
/// # use weighted_list::*;
/// 
/// let list: WeightedList<String, i32> = wlist![
///     (2, String::from("sup")),
///     (3, String::from("nova")),
///     (5, String::from("shard")),
/// ];
/// 
/// for item in list.iter() {
///     println!("{item}");
/// }
/// 
/// let mut rng = rand::rng();
/// if let Some(result) = list.select_random_value(&mut rng) {
///     println!("{}", result);
/// }
/// ```
/// 
/// ### Tips
/// - Most methods return `&Self` or `&mut Self`, allowing you to chain methods. Here's a contrived example:
/// 
/// ```
/// # use weighted_list::*;
/// 
/// let mut list = wlist![(2, "sup")];
/// 
/// list.push_value("sup")
///     .merge_duplicates()
///     .prune()
///     .len();
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
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
    pub fn init<I>(items: I) -> Self
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

/// Construct a `WeightedList` from the provided (weight, value) pairs.
/// 
/// ### Usage
/// ```
/// # use weighted_list::*;
/// 
/// let list = wlist![
///     (2, String::from("sup")),
///     (3, String::from("nova")),
///     (5, String::from("shard")),
/// ];
/// ```
#[macro_export]
macro_rules! wlist {
    ( $( $item: expr ),* $(,)? ) => {
        WeightedList::init([
            $( $item, )*
        ]);
    };
}

// == ACCESSORS == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Get an iterator over copies of the weights of each item in the list.
    pub fn weights(&self) -> impl Iterator<Item = W>
    {
        self.data.iter().map(|item| item.weight)
    }

    /// Get an iterator over references to the values of each item in the list.
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
    /// ```
    /// # use weighted_list::*;
    /// 
    /// let wl = wlist![(2, "sup"), (3, "nova")];
    /// let rl = WeightedList::init(wl.raw());
    /// 
    /// for (left, right) in std::iter::zip(wl.clone(), rl.clone()) {
    ///     assert_eq!(left.weight, right.weight);
    ///     assert_eq!(left.value, *right.value);
    /// }
    /// ```
    pub fn raw(&self) -> impl Iterator<Item = (W,&V)>
    {
        self.data.iter().map(|item| (item.weight, &item.value))
    }
}

// == PROPERTIES == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Sum the weights of all items in the list.
    /// 
    /// ### Notes
    /// - This is not the number of items in the list – use `.total_values()` for that.
    /// - `len() == 0` does not imply the list is empty – items may have zero or negative weights! To check if the list is empty, use `.is_empty()` instead.
    pub fn len(&self) -> W
    {
        self.data.iter().map(|item| item.weight).sum()
    }

    pub fn capacity(&self) -> usize
    {
        self.data.capacity()
    }

    /// How many items/values are in the list?
    /// 
    /// Note that this is not equivalent to `.len()`, which is the total weights of all items in the list.
    pub fn total_values(&self) -> usize
    {
        self.data.len()
    }

    /// Does the list contain no items?
    /// 
    /// Note that this returns `false` if the list contains items with weights of `0`.
    pub fn is_empty(&self) -> bool
    {
        self.data.is_empty()
    }

    /// Do any items have a weight of `0`?
    pub fn is_zero(&self) -> bool
    {
        !self.is_empty()
        && self.data.iter().all(|item| item.weight == W::zero())
    }

    /// Do any items have a negative weight?
    pub fn has_negative_weights(&self) -> bool
    {
        !self.is_empty()
        && self.data.iter().any(|item| item.weight < W::zero())
    }
}

// == CONVERSIONS == //
impl<V, W: Weight> FromIterator<WeightedItem<V,W>> for WeightedList<V,W>
{
    fn from_iter<I>(items: I) -> Self
        where I: IntoIterator<Item = WeightedItem<V,W>>
    {
        // TODO benchmark
        // Self {
        //     data: items.into_iter().collect::<Vec<WeightedItem<V,W>>>()
        // }

        let mut data = vec![];

        for item in items {
            data.push(item);
        }

        Self { data }
    }
}

impl<V, W: Weight> From<Vec<WeightedItem<V,W>>> for WeightedList<V,W> {
    fn from(vec: Vec<WeightedItem<V,W>>) -> Self {
        Self { data: vec }
    }
}

impl<V, W: Weight> Into<Vec<WeightedItem<V,W>>> for WeightedList<V,W> {
    fn into(self) -> Vec<WeightedItem<V,W>> {
        self.data
    }
}

impl<V, W: Weight> AsRef<Vec<WeightedItem<V,W>>> for WeightedList<V,W> {
    fn as_ref(&self) -> &Vec<WeightedItem<V,W>> {
        &self.data
    }
}

impl<V, W: Weight> Deref for WeightedList<V,W> {
    type Target = [WeightedItem<V,W>];

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<V, W: Weight> DerefMut for WeightedList<V,W>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}

// == TRAITS == //
impl<V, W: Weight> Default for WeightedList<V, W> {
    fn default() -> Self {
        Self::new()
    }
}

// == INTERNAL == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Convert a `weighted_index` to its unweighted equivalent in the underlying `Vec`. Does not panic on overflow and instead returns the `.len()` of the underlying `Vec`.
    fn unweight_index_nopanic(&self, weighted_index: W) -> usize
    {
        let mut t = W::zero();
        let mut i = 0;

        for item in &self.data {
            t += item.weight;

            if t > weighted_index {
                return i;
            }

            i += 1;
        }

        i
    }

    /// Convert a `weighted_index` to its unweighted equivalent in the underlying `Vec`. Panics on overflow.
    fn unweight_index(&self, weighted_index: W) -> usize
    {
        let mut t = W::zero();
        let mut i = 0;

        for item in &self.data {
            t += item.weight;

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
impl<V, W: Weight> WeightedList<V,W> {
    pub fn iter(&self) -> impl Iterator<Item = &WeightedItem<V,W>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut WeightedItem<V,W>> {
        self.data.iter_mut()
    }
}

impl<V, W: Weight> IntoIterator for WeightedList<V,W>
{
    type Item = WeightedItem<V,W>;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    /// ```compile_fail
    /// # use weighted_list::*;
    /// 
    /// let list = wlist![]
    /// for _ in list {}
    /// list;  // compile error
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'l, V, W: Weight> IntoIterator for &'l WeightedList<V,W>
{
    type Item = &'l WeightedItem<V,W>;
    type IntoIter = std::slice::Iter<'l, WeightedItem<V,W>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'l, V, W: Weight> IntoIterator for &'l mut WeightedList<V,W>
{
    type Item = &'l mut WeightedItem<V,W>;
    type IntoIter = std::slice::IterMut<'l, WeightedItem<V,W>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

// == LIST MUTATION == //
impl<V, W: Weight> WeightedList<V,W>
{
    pub fn reserve(&mut self, additional: usize) -> &mut Self
    {
        self.data.reserve(additional);
        self
    }

    pub fn push_item(&mut self, item: WeightedItem<V,W>) -> &mut Self
    {
        self.data.push(item);
        self
    }

    pub fn push_new_item(&mut self, weight: W, value: V) -> &mut Self
    {
        self.push_item(WeightedItem { weight, value })
    }
    
    pub fn push_value(&mut self, value: V) -> &mut Self
    {
        self.push_item(WeightedItem::unit(value))
    }

    /// Insert a `WeightedItem` into the list at `weighted_index`. If `weighted_index >= len`, the item is appended to the end (the function does *not* panic).
    pub fn insert_item(&mut self,
        weighted_index: W,
        item: WeightedItem<V,W>
    ) -> &mut Self
    {
        self.data.insert(self.unweight_index_nopanic(weighted_index), item);
        self
    }

    /// Insert an item with `weight` and `value` into the list at `weighted_index`. If `weighted_index >= len`, the item is appended to the end (the function does *not* panic).
    pub fn insert_new_item(&mut self,
        weighted_index: W,
        (weight, value): (W, V)
    ) -> &mut Self
    {
        self.insert_item(weighted_index, WeightedItem::new(weight, value))
    }

    /// Insert an item with `value` and a weight of `1` into the list at `weighted_index`. If `weighted_index >= len`, the item is appended to the end (the function does *not* panic).
    pub fn insert_value(&mut self,
        weighted_index: W,
        value: V,
    ) -> &mut Self
    {
        self.insert_item(weighted_index, WeightedItem::unit(value))
    }

    pub fn append(&mut self, other: &mut WeightedList<V,W>) -> &mut Self
    {
        self.data.append(&mut other.data);
        self
    }

    pub fn reverse(&mut self) -> &mut Self
    {
        self.data.reverse();
        self
    }

    pub fn swap(&mut self, left: W, right: W) -> &mut Self
    {
        let l = self.unweight_index(left);
        let r = self.unweight_index(right);
        self.data.swap(l, r);
        self
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

    /// Remove the entire item at `weighted_index` and return it.
    pub fn remove(&mut self, weighted_index: W) -> WeightedItem<V,W>
    {
        self.data.remove(self.unweight_index(weighted_index))
    }

    // UNTESTED
    pub fn truncate(&mut self, len: W) -> &mut Self
    {
        let mut t = W::zero();
        
        for each in &mut self.data {
            t += each.weight;

            if t > len {
                each.weight = t - len;
            }
        }

        self
    }

    pub fn retain<F>(&mut self, predicate: F) -> &mut Self
        where F: FnMut(&WeightedItem<V,W>) -> bool
    {
        self.data.retain(predicate);
        self
    }

    pub fn retain_mut<F>(&mut self, predicate: F) -> &mut Self
        where F: FnMut(&mut WeightedItem<V,W>) -> bool
    {
        self.data.retain_mut(predicate);
        self
    }

    /// Clear the `WeightedList`, removing all items.
    pub fn clear(&mut self) -> &mut Self
    {
        self.data.clear();
        self
    }
}

impl<V: Clone, W: Weight> WeightedList<V,W>
{
    pub fn sorted(&self) -> Self
        where V: Eq, W: Ord
    {
        let mut out = self.clone();
        out.sort();
        out
    }
    
    pub fn reversed(&self) -> Self
    {
        let mut out = self.clone();
        out.reverse();
        out
    }
}

// == SPECIALISED MUTATION == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Remove all items with non-positive weight.
    pub fn prune(&mut self) -> &mut Self
    {
        self.data.retain(|item| item.weight > W::zero());
        self
    }

    /// Set the weight of all items to `0`.
    pub fn zero_all_weights(&mut self) -> &mut Self
    {
        for item in &mut self.data {
            item.weight = W::zero();
        }

        self
    }

    /// Set the weight of all items to `weight`.
    pub fn set_all_weights(&mut self, weight: W) -> &mut Self
    {
        for item in &mut self.data {
            item.weight = weight;
        }

        self
    }

    pub fn normalised(&mut self) -> Result<WeightedList<V, f64>, &str>
        where V: Clone
    {
        let total;

        if let Some(t) = nums::cast::<W, f64>(self.len()) {
            total = t;
        } else {
            return Err("Error casting total weights to `f64`");
        };

        let items =
            self.data
                .iter()
                .map(|item| {
                    let weight = nums::cast::<W, f64>(item.weight)?;
                    Some(WeightedItem {
                        weight: weight / total,
                        value: item.value.clone()
                    })
                })
                .collect::<Option<Vec<WeightedItem<V, f64>>>>()
        ;

        match items {
            Some(data) => Ok(WeightedList { data }),
            None       => Err("Error casting weights to `f64`")
        }
    }
}

impl<V: PartialEq, W: Weight> WeightedList<V,W>
{
    /// Merge a `WeightedItem` into the list. If an item with the same value already exists, add the weight of the new item to the existing item. Otherwise, append the new item to the list.
    /// 
    /// ### Tips
    /// - Use this method when you already have an existing `WeightedItem` instance. If you're going to construct a new `WeightedItem`, `.merge_new_item()` will be more convenient.
    /// 
    /// ### Usage
    /// ```
    /// # use weighted_list::*;
    /// 
    /// let mut wl = wlist![(1, "sup")];
    /// 
    /// let item = WeightedItem::new(2, "sup");
    /// wl.merge_item(item);
    /// assert!(wl == wlist![(3, "sup")]);
    /// // "sup" merged with existing instance
    /// 
    /// let item = WeightedItem::unit("elysion");
    /// wl.merge_item(item);
    /// assert!(wl == wlist![(3, "sup"), (1, "elysion")]);
    /// // "elysion" appended to end
    /// ```
    pub fn merge_item(&mut self, item: WeightedItem<V,W>) -> &mut Self
    {
        if let Some(existing) = self.data.iter_mut().find(|each| each.value == item.value) {
            existing.weight += item.weight;
        }
        else {
            self.data.push(item);
        }

        self
    }

    /// Merge an item with `value` and `weight` into the list.
    /// 
    /// See `.merge_item()` for details.
    pub fn merge_new_item(&mut self, weight: W, value: V) -> &mut Self
    {
        self.merge_item(WeightedItem { weight, value })
    }

    /// Merge an item with `value` and a weight of `1` into the list.
    /// 
    /// See `.merge_item()` for details.
    pub fn merge_value(&mut self, value: V) -> &mut Self
    {
        self.merge_item(WeightedItem::unit(value))
    }

    pub fn merge_with(&mut self, other: WeightedList<V,W>) -> &mut Self
    {
        for item in other {
            self.merge_item(item);
        }

        self
    }

    pub fn merge_duplicates(&mut self) -> &mut Self
    {
        let orig = std::mem::replace(self, WeightedList::new());
        self.merge_with(orig);
        self
    }
}

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

    pub fn take_entire_random<RNG>(&mut self, rng: &mut RNG) -> Option<WeightedItem<V,W>>
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
    /// Select `count` items using weighted randomisation.
    /// 
    /// Call this method using `bon` builder syntax (see § Usage below).
    /// 
    /// ### Parameters
    /// - `count`: How many values to select.
    /// - `replace`: If `false`, items have their weight decremented after selection. If `true`, infinite values can be selected.
    ///   - `decrement`: How much to decrement weights by if `replace` is `false`. Defaults to `1`.
    /// - `unique`: If `true`, only distinct values will be returned. `replace` becomes irrelevant in this case.
    /// 
    /// ### Notes
    /// - If `count` exceeds the length of the list, excess iterations will be skipped. If selection for an iteration fails, the values is excluded from the output list. Note that these mean the results may have fewer values than the expected `count`.
    /// - This method reserves a `Vec<>` with capacity `count` initially, so be careful of passing in extremely large `count`s.
    /// 
    /// ### Usage
    /// This method uses the bon builder syntax:
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let list = wlist![
    ///     (2, String::from("sup")),
    ///     (3, String::from("nova")),
    ///     (5, String::from("shard")),
    /// ];
    /// 
    /// let mut rng = rand::rng();
    /// 
    /// // with replacement
    /// let selected =
    ///     list.select_random_values()
    ///         .rng(&mut rng)
    ///         .count(3)
    ///         .call();
    /// 
    /// assert!(selected.len() == 3);
    /// 
    /// // without replacement
    /// let selected =
    ///     list.select_random_values()
    ///         .rng(&mut rng)
    ///         .count(10)
    ///         .replace(false)
    ///         .decrement(2)
    ///         .call();
    /// 
    /// assert!(selected.len() == 6);
    /// 
    /// // unique only
    /// let selected =
    ///     list.select_random_values()
    ///         .rng(&mut rng)
    ///         .count(100)
    ///         .unique(true)
    ///         .call();
    /// 
    /// assert!(selected.len() == 3);
    /// ```
    #[builder]
    pub fn select_random_values<RNG>(&self,
        count: u32,
        rng: &mut RNG,
        replace: Option<bool>,
            decrement: Option<W>,
        unique: Option<bool>,
    ) -> Vec<V>
        where RNG: Rng + ?Sized
    {
        let unique = unique.unwrap_or(false);
        let replace = replace.unwrap_or(true);
        let decrement = decrement.unwrap_or(W::one());

        let mut pool = self.clone();
        let mut i = 0;
        let mut out = Vec::with_capacity(count as usize);

        loop
        {
            i += 1;
            if i > count { break }

            if let Some(item) = {
                if unique       { pool.take_entire_random(rng) }
                else if replace { pool.take_by_random(rng, W::zero()) }
                else            { pool.take_by_random(rng, decrement) }
            } {
                out.push(item.value.clone());
            }
            else {
                continue
            }
        }

        out
    }
}

impl<V: Clone, W: Weight + Clone> WeightedList<V,W>
{
    pub fn shuffle_items<RNG>(&mut self, rng: &mut RNG) -> &mut Self
        where RNG: Rng + ?Sized
    {
        self.data.shuffle(rng);
        self
    }

    pub fn shuffled_items<RNG>(&self, rng: &mut RNG) -> Self
        where RNG: Rng + ?Sized
    {
        let mut out = self.clone();
        out.shuffle_items(rng);

        out
    }

    pub fn shuffle_weights<RNG>(&mut self, rng: &mut RNG) -> &mut Self
        where RNG: Rng + ?Sized
    {
        let mut weights: Vec<W> = self.weights().collect();
        weights.shuffle(rng);
        
        for item in &mut self.data {
            /* guaranteed to be Some */
            item.weight = weights.pop().unwrap();
        }

        self
    }

    pub fn shuffled_weights<RNG>(&self, rng: &mut RNG) -> Self
        where RNG: Rng + ?Sized
    {
        let mut out = self.clone();
        out.shuffle_weights(rng);

        out
    }
}
