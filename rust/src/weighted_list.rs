use std::{
    fmt,
    iter,
    ops::*,
};

use bon::{bon};
use itertools::Itertools;
use num_traits as nums;
use rand::{
    prelude::*,
    seq::SliceRandom,
};

use crate::root::*;
use crate::WeightedItem;


/// A homogeneous list of weighted items with values of type `V` and weights of numerical type `W`.
/// 
/// Near-identical to `Vec<T>`, but stores `WeightedItem<V,W>` objects instead. You can think of it like a `Vec<WeightedItem<V,W>>`.
/// 
/// # Usage
/// 
/// ```
/// # use weighted_list::*;
/// let wl: WeightedList<String, i32> = wlist![
///     (2, "sup".to_string()),
///     (3, "nova".to_string()),
///     (5, "shard".to_string()),
/// ];
/// 
/// for item in &wl {
///     println!("{item}");
/// }
/// 
/// if let Some(result) = wl.select_random_value(&mut rand::rng()) {
///     println!("{}", result);
/// }
/// ```
/// 
/// # Indexing
/// 
/// `WeightedList` uses *weighted* indexing; this is the key difference between it and a `Vec`. It's most easily explained with an example:
/// 
/// ```should_panic
/// # use weighted_list::*;
/// let wl = wlist![(1, "qi"), (2, "sup"), (5, "shard")];
/// 
/// let _ = wl[0]; // => WeightedItem { weight: 1, value: "qi" }
/// let _ = wl[1]; // => WeightedItem { weight: 2, value: "sup" }
/// let _ = wl[2]; // => WeightedItem { weight: 2, value: "sup" }
/// let _ = wl[3]; // => WeightedItem { weight: 5, value: "shard" }
/// let _ = wl[4]; // => WeightedItem { weight: 5, value: "shard" }
/// let _ = wl[5]; // => WeightedItem { weight: 5, value: "shard" }
/// let _ = wl[6]; // => WeightedItem { weight: 5, value: "shard" }
/// let _ = wl[7]; // => WeightedItem { weight: 5, value: "shard" }
/// let _ = wl[8]; // => panic - out of bounds!
/// ```
/// 
/// In essence, each value is "copied" a number of times equal to its weight – this is what enables the weighted randomisation. But because the values are stored in `WeightedItem` objects, instead of actually being copied, larger weight values can be used without fear of performance impacts.
/// 
/// # Tips
/// 
/// - Most methods return `&Self` or `&mut Self`, allowing you to chain methods. Here's a contrived example:
/// 
/// ```
/// # use weighted_list::*;
/// let mut list = wlist![(2, "sup")];
/// 
/// list.push_value("sup")
///     .merge_duplicates()
///     .prune()
///     .len();
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct WeightedList<V, W: Weight>
{
    data: Vec<WeightedItem<V,W>>
}

// == CONSTRUCTORS == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Construct an empty `WeightedList`.
    pub fn new() -> Self
    {
        Self { data: Vec::new() }
    }

    /// Construct an empty `WeightedList` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self
    {
        Self { data: Vec::with_capacity(capacity) }
    }

    /// Construct a `WeightedList` from an iterable of `(weight, value)` pairs.
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

/// Construct a `WeightedList` from the provided `(weight, value)` pairs.
/// 
/// # Usage
/// 
/// ```
/// # use weighted_list::*;
/// let wl = wlist![
///     (2, "sup"),
///     (3, "nova"),
///     (5, "shard"),
/// ];
/// 
/// let empty: WeightedList<(), usize> = wlist![];
/// ```
#[macro_export]
macro_rules! wlist {
    ( $( $item: expr ),* $(,)? ) => {
        WeightedList::init([
            $( $item, )*
        ])
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

    /// Get a reference to the `Vec<>` of items in the list.
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

impl<V, W: Weight + nums::PrimInt> WeightedList<V,W>
{
    pub fn expanded(&self) -> impl Iterator<Item = &V>
    {
        self.data
            .iter()
            .flat_map(|item| iter::repeat_n(
                &item.value,
                nums::cast::<W, usize>(item.weight).unwrap_or(0)
            ))
    }
}

// == PROPERTIES == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Sum the weights of all items in the list.
    /// 
    /// # Notes
    /// 
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

impl<V, W: Weight> From<Vec<WeightedItem<V,W>>> for WeightedList<V,W>
{
    fn from(vec: Vec<WeightedItem<V,W>>) -> Self {
        Self { data: vec }
    }
}

impl<V, W: Weight> From<WeightedList<V,W>> for Vec<WeightedItem<V,W>>
{
    fn from(list: WeightedList<V,W>) -> Self {
        list.data
    }
}

impl<V, W: Weight> AsRef<Vec<WeightedItem<V,W>>> for WeightedList<V,W>
{
    fn as_ref(&self) -> &Vec<WeightedItem<V,W>> {
        &self.data
    }
}

impl<V, W: Weight> Deref for WeightedList<V,W>
{
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
impl<V: fmt::Display, W: Weight + fmt::Display> fmt::Display for WeightedList<V,W>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {            
        write!(f,
            "WeightedList[{}]",
            self.data.iter().map(|item| item.to_string()).join(", ")
        )
    }
}

impl<V, W: Weight> Extend<WeightedItem<V,W>> for WeightedList<V,W>
{
    fn extend<T>(&mut self, iter: T)
        where T: IntoIterator<Item = WeightedItem<V,W>>
    {
        for item in iter {
            self.push_item(item);
        }
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
    fn index_mut(&mut self, weighted_index: W) -> &mut Self::Output
    {
        let idx = self._unweight_index_(weighted_index);
        &mut self.data[idx]
    }
}

// == ITERATION == //
impl<V, W: Weight> IntoIterator for WeightedList<V,W>
{
    type Item = WeightedItem<V,W>;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    /// ```compile_fail
    /// # use weighted_list::*;
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

    /// Append an item to the end of the list.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     *wl.push_item(wl[0].clone()),
    ///     wlist![(2, "sup"), (3, "nova"), (5, "shard"), (2, "sup")],
    /// )
    /// ```
    pub fn push_item(&mut self, item: WeightedItem<V,W>) -> &mut Self
    {
        self.data.push(item);
        self
    }

    /// Append a new item with `value` and `weight` to the end of the list.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     *wl.push_new_item(7, "cortex"),
    ///     wlist![(2, "sup"), (3, "nova"), (5, "shard"), (7, "cortex")],
    /// )
    /// ```
    pub fn push_new_item(&mut self, weight: W, value: V) -> &mut Self
    {
        self.push_item(WeightedItem { weight, value })
    }
    
    /// Append a new item with `value` and a weight of `1` to the end of the list.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     *wl.push_value("elysion"),
    ///     wlist![(2, "sup"), (3, "nova"), (5, "shard"), (1, "elysion")],
    /// )
    /// ```
    pub fn push_value(&mut self, value: V) -> &mut Self
    {
        self.push_item(WeightedItem::unit(value))
    }

    /// Insert an item into the list at `weighted_index`. If `weighted_index >= len`, the item is appended to the end (the function does *not* panic).
    pub fn insert_item(&mut self,
        weighted_index: W,
        item: WeightedItem<V,W>
    ) -> &mut Self
    {
        self.data.insert(self._unweight_index_nopanic_(weighted_index), item);
        self
    }

    /// Insert a new item with `value` and `weight` into the list at `weighted_index`. If `weighted_index >= len`, the item is appended to the end (the function does *not* panic).
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

    /// Move all items in `other` into `self`, leaving `other` empty.
    pub fn append(&mut self, other: &mut WeightedList<V,W>) -> &mut Self
    {
        self.data.append(&mut other.data);
        self
    }

    /// Reverse the order of items in the list.
    pub fn reverse(&mut self) -> &mut Self
    {
        self.data.reverse();
        self
    }

    /// Swap the items at weighted indices `left` and `right`.
    /// 
    /// # Panics
    /// 
    /// Panics if `left` or `right` are out of bounds.
    pub fn swap(&mut self, left: W, right: W) -> &mut Self
    {
        let l = self._unweight_index_(left);
        let r = self._unweight_index_(right);
        self.data.swap(l, r);
        self
    }

    /// Removes the last item from the list and returns it, or `None` if the list is empty.
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
    /// 
    /// # Panics
    /// 
    /// Panics if `weighted_index` is out of bounds.
    pub fn remove_at(&mut self, weighted_index: W) -> WeightedItem<V,W>
    {
        self.data.remove(self._unweight_index_(weighted_index))
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

    /// Retain only items that fulfil the predicate.
    pub fn retain<F>(&mut self, predicate: F) -> &mut Self
        where F: FnMut(&WeightedItem<V,W>) -> bool
    {
        self.data.retain(predicate);
        self
    }

    /// Retain only items that fulfil the predicate, passing a mutable reference to the predicate.
    pub fn retain_mut<F>(&mut self, predicate: F) -> &mut Self
        where F: FnMut(&mut WeightedItem<V,W>) -> bool
    {
        self.data.retain_mut(predicate);
        self
    }

    /// Clear the list, removing all items.
    /// 
    /// If you'd like to set the weights of all items to `0`, you can use `.zero_all_weights()`.
    pub fn clear(&mut self) -> &mut Self
    {
        self.data.clear();
        self
    }
}

impl<V: Clone, W: Weight> WeightedList<V,W>
{
    /// Return a clone of the list with items sorted in ascending order of weights.
    /// 
    /// Orderings of items with equivalent weights is (currently) undefined behaviour.
    pub fn sorted(&self) -> Self
        where V: Eq, W: Ord
    {
        let mut out = self.clone();
        out.sort();
        out
    }
    
    /// Return a clone of the list with items reversed.
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

    pub fn pruned(&self) -> Self
        where V: Clone
    {
        let mut out = self.clone();
        out.prune();
        out
    }

    /// Find the first occurrence (from the left) of an item with `value`, and remove the entire item.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     *wl.remove_value_first(&"nova"),
    ///     wlist![(2, "sup"), (5, "shard")],
    /// )
    /// ```
    pub fn remove_value_first(&mut self, value: &V) -> &mut Self
        where V: PartialEq
    {
        self.remove_first_where(|item| item.value == *value)
    }

    /// Find the last occurrence (from the right) of an item with `value`, and remove the entire item.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(0, "qi"), (1, "qi"), (2, "sup")];
    /// 
    /// assert_eq!(
    ///     *wl.remove_value_last(&"qi"),
    ///     wlist![(0, "qi"), (2, "sup")],
    /// )
    /// ```
    pub fn remove_value_last(&mut self, value: &V) -> &mut Self
        where V: PartialEq
    {
        self.remove_last_where(|item| item.value == *value)
    }

    /// Find the first occurrence (from the left) of an item that matches `predicate`, and remove the entire item.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     *wl.remove_first_where(|item| item.weight > 2),
    ///     wlist![(2, "sup"), (5, "shard")],
    /// )
    /// ```
    pub fn remove_first_where<F>(&mut self, predicate: F) -> &mut Self
        where F: FnMut(&WeightedItem<V,W>) -> bool
    {
        if let Some(idx) = self.iter().position(predicate) {
            self.data.remove(idx);
        }

        self
    }

    /// Find the last occurrence (from the right) of an item that matches `predicate`, and remove the entire item.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     *wl.remove_last_where(|item| item.weight > 2),
    ///     wlist![(2, "sup"), (3, "nova")],
    /// );
    /// ```
    pub fn remove_last_where<F>(&mut self, predicate: F) -> &mut Self
        where F: FnMut(&WeightedItem<V,W>) -> bool
    {
        if let Some(idx) = self.iter().rposition(predicate) {
            self.data.remove(idx);
        }

        self
    }

    /// Set the weight of all items to `0`.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     *wl.zero_all_weights(),
    ///     wlist![(0, "sup"), (0, "nova"), (0, "shard")],
    /// )
    /// ```
    pub fn zero_all_weights(&mut self) -> &mut Self
    {
        for item in &mut self.data {
            item.weight = W::zero();
        }

        self
    }

    /// Set the weight of all items to `weight`.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     *wl.set_all_weights(1),
    ///     wlist![(1, "sup"), (1, "nova"), (1, "shard")],
    /// )
    /// ```
    pub fn set_all_weights(&mut self, weight: W) -> &mut Self
    {
        for item in &mut self.data {
            item.weight = weight;
        }

        self
    }

    /// Return a clone of the list with all item weights normalised such that they sum to `1.0`.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// assert_eq!(
    ///     wl.normalised().ok(),
    ///     Some(wlist![(0.2, "sup"), (0.3, "nova"), (0.5, "shard")])
    /// );
    /// ```
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
    /// # Tips
    /// 
    /// - Use this method when you already have an existing `WeightedItem` instance. If you're going to construct a new `WeightedItem`, `.merge_new_item()` will be more convenient.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
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

    /// Merge a new item with `value` and `weight` into the list.
    /// 
    /// See `.merge_item()` for details.
    pub fn merge_new_item(&mut self, weight: W, value: V) -> &mut Self
    {
        self.merge_item(WeightedItem { weight, value })
    }

    /// Merge a new item with `value` and a weight of `1` into the list.
    /// 
    /// See `.merge_item()` for details.
    pub fn merge_value(&mut self, value: V) -> &mut Self
    {
        self.merge_item(WeightedItem::unit(value))
    }

    /// Merge the items of `other` into `self`, leaving `other` empty.
    /// 
    /// See `.merge_item()` for details.
    pub fn merge_with(&mut self, other: WeightedList<V,W>) -> &mut Self
    {
        for item in other {
            self.merge_item(item);
        }

        self
    }

    /// Merge any duplicate items in the list.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![
    ///     (1, "sup"),
    ///     (2, ""),
    ///     (20, "sup")
    /// ];
    /// 
    /// assert_eq!(
    ///     *wl.merge_duplicates(),
    ///     wlist![
    ///         (21, "sup"),
    ///         (2, ""),
    ///     ]
    /// );
    /// ```
    pub fn merge_duplicates(&mut self) -> &mut Self
    {
        let orig = std::mem::replace(self, WeightedList::new());
        self.merge_with(orig);
        self
    }
}

impl<V: Clone, W: Weight> WeightedList<V,W>
{
    /// Decrement the weight of the item at `weighted_index` by `1`. If its weight becomes non-positive as a result, remove the entire item. Returns a clone of the item with its updated weight.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.take_one_at(2);
    /// assert_eq!( wl, wlist![(2, "sup"), (2, "nova"), (5, "shard")] );
    /// 
    /// wl.take_one_at(2);
    /// assert_eq!( wl, wlist![(2, "sup"), (1, "nova"), (5, "shard")] );
    /// 
    /// wl.take_one_at(2);
    /// assert_eq!( wl, wlist![(2, "sup"), (5, "shard")] );
    /// ```
    pub fn take_one_at(&mut self, weighted_index: W) -> WeightedItem<V,W>
    {
        self.take_by_at(W::one(), weighted_index)
    }

    /// Decrement the weight of the item at `weighted_index` by `decrement`. If its weight becomes non-positive as a result, remove the entire item. Returns a clone of the item with its updated weight.
    /// 
    /// # Panics
    /// 
    /// Panics if `weighted_index` is out of bounds.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.take_by_at(2, 2);
    /// assert_eq!( wl, wlist![(2, "sup"), (1, "nova"), (5, "shard")] );
    /// 
    /// wl.take_by_at(2, 2);
    /// assert_eq!( wl, wlist![(2, "sup"), (5, "shard")]);
    /// ```
    pub fn take_by_at(&mut self, decrement: W, weighted_index: W) -> WeightedItem<V,W>
    {
        let idx = self._unweight_index_(weighted_index);
        let target = &mut self.data[idx];

        target.weight -= decrement;

        if target.weight <= W::zero() {
            self.data.remove(idx)
        }
        else {
            target.clone()
        }
    }

    /// Remove the entire item at `weighted_index`.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.take_entire_at(3);
    /// assert_eq!( wl, wlist![(2, "sup"), (5, "shard")] );
    /// ```
    pub fn take_entire_at(&mut self, weighted_index: W) -> WeightedItem<V,W>
    {
        self.remove_at(weighted_index)
    }
}

// == RANDOMISATION == //
impl<V, W: Weight> WeightedList<V,W>
{
    fn _get_random_weighted_index_up_to_<RNG>(&self, rng: &mut RNG, upper: W) -> Option<W>
        where RNG: Rng + ?Sized
    {
        let len:    f64 = nums::cast::<W, f64>(upper)?;
        let scalar: f64 = rng.random();

        let idx = (len * scalar).floor();
        let out = nums::cast::<f64, W>(idx)?;

        Some(out)
    }

    fn _get_random_weighted_index_<RNG>(&self, rng: &mut RNG) -> Option<W>
        where RNG: Rng + ?Sized
    {
        self._get_random_weighted_index_up_to_(rng, self.len())
    }

    /// Select a random item from the list and return its value, using weighted randomisation.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.select_random_value(&mut rand::rng());
    /// // could give:
    /// //   - Some("sup"  ) with 20% probability
    /// //   - Some("nova" ) with 30% probability
    /// //   - Some("shard") with 50% probability
    /// ```
    pub fn select_random_value<RNG>(&self, rng: &mut RNG) -> Option<&V>
        where RNG: Rng + ?Sized
    {
        let out = &self.select_random_item(rng)?.value;
        Some(out)
    }

    /// Select a random item from the list, using weighted randomisation.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.select_random_item(&mut rand::rng());
    /// // could give:
    /// //   - Some(WeightedItem { 2, "sup"   }) with 20% probability
    /// //   - Some(WeightedItem { 3, "nova"  }) with 30% probability
    /// //   - Some(WeightedItem { 5, "shard" }) with 50% probability
    /// ```
    pub fn select_random_item<RNG>(&self, rng: &mut RNG) -> Option<&WeightedItem<V,W>>
        where RNG: Rng + ?Sized
    {
        if self.data.is_empty() { return None }

        let idx = self._get_random_weighted_index_(rng)?;
        let out = &self[idx];

        Some(out)
    }
}

impl<V: Clone, W: Weight> WeightedList<V,W>
{
    /// Select a random item from the list using weighted randomisation, and decrement its weight by `1`.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.take_one_random(&mut rand::rng());
    /// // could give:
    /// //   - Some(WeightedItem { 1, "sup"   })   with 20% probability
    /// //   - Some(WeightedItem { 2, "nova"  })  with 30% probability
    /// //   - Some(WeightedItem { 4, "shard" }) with 50% probability
    /// ```
    pub fn take_one_random<RNG>(&mut self, rng: &mut RNG) -> Option<WeightedItem<V,W>>
        where RNG: Rng + ?Sized
    {
        self.take_by_random(rng, W::one())
    }

    /// Select a random item from the list using weighted randomisation, and decrement its weight by `decrement`.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.take_by_random(&mut rand::rng(), 2);
    /// // could give:
    /// //   - Some(WeightedItem { 0, "sup"   })   with 20% probability
    /// //   - Some(WeightedItem { 1, "nova"  })  with 30% probability
    /// //   - Some(WeightedItem { 3, "shard" }) with 50% probability
    /// ```
    pub fn take_by_random<RNG>(&mut self, rng: &mut RNG, decrement: W) -> Option<WeightedItem<V,W>>
        where RNG: Rng + ?Sized
    {
        if self.data.is_empty() { return None }

        let idx = self._get_random_weighted_index_(rng)?;
        let out = self.take_by_at(decrement, idx);

        Some(out)
    }

    /// Select and remove a random item from the list, using weighted randomisation.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.take_entire_random(&mut rand::rng());
    /// // could give:
    /// //   - Some(WeightedItem { 2, "sup"   })   with 20% probability
    /// //   - Some(WeightedItem { 3, "nova"  })  with 30% probability
    /// //   - Some(WeightedItem { 5, "shard" }) with 50% probability
    /// 
    /// assert!( wl.total_values() == 2 );
    /// ```
    pub fn take_entire_random<RNG>(&mut self, rng: &mut RNG) -> Option<WeightedItem<V,W>>
        where RNG: Rng + ?Sized
    {
        if self.data.is_empty() { return None }

        let idx = self._get_random_weighted_index_(rng)?;
        let out = self.take_entire_at(idx);

        Some(out)
    }
}

#[bon]
impl<V: Clone + Eq, W: Weight> WeightedList<V,W>
{
    /// Select `count` values using weighted randomisation.
    /// 
    /// Call this method using `bon` builder syntax (see § Usage below).
    /// 
    /// # Options
    /// 
    /// ```ignore
    /// rng: RNG,
    /// count: usize,
    /// replace: Option<bool> = true,
    ///     decrement: Option<W> = 1,
    /// unique: Option<bool> = false,
    /// ```
    /// 
    /// - `count`: How many values to select.
    /// - `replace`: If `true`, items do not have their weight decremented after selection, and infinite values can be selected. If `false`, items have their weight decremented after selection.
    ///   - This means at most `self.len()` values will be returned.
    /// - `decrement`: How much to decrement weights by if `replace` is `false`.
    /// - `unique`: If `true`, only distinct values will be returned.
    ///   - `replace` becomes irrelevant in this case.
    ///   - This uses `Eq` equality comparison.
    ///   - This means at most `self.total_values()` values will be returned.
    /// 
    /// # Usage
    /// 
    /// This method uses the bon builder syntax:
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut pool = wlist![
    ///     (2, "sup".to_string()),
    ///     (3, "nova".to_string()),
    ///     (5, "shard".to_string()),
    /// ];
    /// 
    /// let mut rng = rand::rng();
    /// 
    /// // with replacement
    /// let selected =
    ///     pool.select_random_values()
    ///         .rng(&mut rng)
    ///         .count(3)
    ///         .call();
    /// 
    /// assert!(selected.len() == 3);
    /// 
    /// // without replacement
    /// let selected =
    ///     pool.select_random_values()
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
    ///     pool.select_random_values()
    ///         .rng(&mut rng)
    ///         .count(100)
    ///         .unique(true)
    ///         .call();
    /// 
    /// assert!(selected.len() == 3);
    /// ```
    /// 
    /// # Notes
    /// 
    /// - It is not guaranteed that the results will have exactly `count` values.
    ///   - If `count` exceeds the maximum possible number of values that can be returned, excess iterations will be skipped.
    ///   - If selection for an iteration fails, that value is excluded from the output list.
    /// - This method reserves a `Vec<>` with capacity `count` initially, so be careful of passing in extremely large `count`s.
    #[builder]
    pub fn select_random_values<RNG>(&self,
        rng: &mut RNG,
        count: usize,
        replace: Option<bool>,
            decrement: Option<W>,
        unique: Option<bool>,
    ) -> Vec<V>
        where RNG: Rng + ?Sized
    {
        let replace = replace.unwrap_or(true);
        let decrement = decrement.unwrap_or(W::one());
        let unique = unique.unwrap_or(false);

        let mut pool = self.clone();
        let mut n = 0;
        let mut out = Vec::with_capacity(count);

        loop
        {
            n += 1;
            if n > count || self.data.is_empty() { break }

            if let Some(item) = {
                if unique       { pool.take_entire_random(rng) }
                else if replace { pool.take_by_random(rng, W::zero()) }
                else            { pool.take_by_random(rng, decrement) }
            } {
                out.push(item.value.clone());
            }
        }

        out
    }

    /// Take `count` values using weighted randomisation.
    #[builder]
    pub fn take_random_values<RNG>(&mut self,
        rng: &mut RNG,
        count: usize,
        take_entire: Option<bool>,
            decrement: Option<W>,
    ) -> Vec<V>
        where RNG: Rng + ?Sized
    {
        let take_entire = take_entire.unwrap_or(true);
        let decrement = decrement.unwrap_or(W::one());

        let mut n = 0;
        let mut out = Vec::with_capacity(count as usize);

        loop
        {
            n += 1;
            if n > count || self.data.is_empty() { break }

            if let Some(item) = {
                if take_entire { self.take_entire_random(rng) }
                else           { self.take_by_random(rng, decrement) }
            } {
                out.push(item.value.clone());
            }
        }

        out
    }
}

#[bon]
impl<V: Clone + Eq + std::hash::Hash, W: Weight> WeightedList<V,W>
{
    /// Variant of `._unweighted_index_()` for mutating random selection with unique outputs.
    fn _unweight_index_skipping_(&self,
        weighted_index: W,
        seen: &std::collections::HashSet<V>,
    ) -> Option<usize>
    {
        let mut t = W::zero();

        for (i, item) in self.data.iter().enumerate() {
            if seen.contains(&item.value) {
                continue
            }

            t += item.weight;

            if t > weighted_index {
                return Some(i);
            }
        }

        None
    }

    /// Take `count` values using weighted randomisation.
    #[builder]
    pub fn take_random_values_unique<RNG>(&mut self,
        rng: &mut RNG,
        count: usize,
        decrement: Option<W>,
    ) -> Vec<V>
        where RNG: Rng + ?Sized,
    {
        let decrement = decrement.unwrap_or(W::one());

        let mut n = 0;
        let mut l = self.len();

        let mut seen = std::collections::HashSet::<V>::new();

        let mut out = Vec::with_capacity(
            if count > 16 {
                count.min(self.total_values())
            } else {
                count
            }
        );
        
        loop
        {
            n += 1;
            if n > count || self.data.is_empty() { break }

            if let Some(value) = (|| {
                let weighted_index = self._get_random_weighted_index_up_to_(rng, l)?;
                let idx = self._unweight_index_skipping_(weighted_index, &seen)?;

                let target = &mut self.data[idx];
                let value = target.value.clone();

                target.weight -= decrement;

                if target.weight <= W::zero() {
                    self.data.remove(idx);
                }

                Some(value)
            })()
            {
                seen.insert(value.clone());
                out.push(value.clone());

                l = self.data.iter()
                    .filter_map(
                        |item| {
                            if !seen.contains(&item.value) { Some(item.weight) }
                            else { None }
                        }
                    )
                    .sum::<W>();
            }
        }

        out
    }
}

impl<V: Clone, W: Weight + Clone> WeightedList<V,W>
{
    /// Shuffle the order of items in the list in-place.
    pub fn shuffle_items<RNG>(&mut self, rng: &mut RNG) -> &mut Self
        where RNG: Rng + ?Sized
    {
        self.data.shuffle(rng);
        self
    }

    /// Return a clone with the order of items shuffled.
    /// 
    /// Out-of-place version of `.shuffle_items()`.
    pub fn shuffled_items<RNG>(&self, rng: &mut RNG) -> Self
        where RNG: Rng + ?Sized
    {
        let mut out = self.clone();
        out.shuffle_items(rng);

        out
    }

    /// Shuffle the pairings of (weight, value) for items in the list, in-place.
    /// 
    /// Values remain in the same order, while weights are re-assigned.
    /// 
    /// # Usage
    /// 
    /// ```
    /// # use weighted_list::*;
    /// let mut wl = wlist![(2, "sup"), (3, "nova"), (5, "shard")];
    /// 
    /// wl.shuffle_weights(&mut rand::rng());
    /// 
    /// println!("{wl}");
    /// // could give:
    /// //   WeightedList[{3, sup}, {5, nova}, {2, shard}]
    /// 
    /// println!("{wl}");
    /// // could now give:
    /// //   WeightedList[{2, sup}, {5, nova}, {3, shard}]
    /// ```
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

    /// Return a clone of the list with (weight, value) pairings shuffled.
    /// 
    /// Out-of-place version of `.shuffle_weights()`.
    pub fn shuffled_weights<RNG>(&self, rng: &mut RNG) -> Self
        where RNG: Rng + ?Sized
    {
        let mut out = self.clone();
        out.shuffle_weights(rng);

        out
    }
}

// == INTERNAL == //
impl<V, W: Weight> WeightedList<V,W>
{
    /// Convert a `weighted_index` to its unweighted equivalent in the underlying `Vec`. Does not panic on overflow and instead returns the `.len()` of the underlying `Vec`.
    fn _unweight_index_nopanic_(&self, weighted_index: W) -> usize
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
    fn _unweight_index_(&self, weighted_index: W) -> usize
    {
        let mut t = W::zero();

        for (i, item) in self.data.iter().enumerate() {
            t += item.weight;

            if t > weighted_index {
                return i;
            }
        }

        panic!(
            "index out of bounds: the len is {} but the index is {}",
            self.len(), weighted_index
        );
    }
}


#[cfg(test)] mod tests
{
    use super::*;

    fn el() -> WeightedList<String, i32>
    {
        wlist![]
    }

    fn wl() -> WeightedList<String, i32>
    {
        wlist![
            (2, "sup".to_string()),
            (3, "nova".to_string()),
            (5, "shard".to_string()),
        ]
    }

    #[test] fn _unweight_index_()
    {
        let list = wl();
        assert_eq!( list._unweight_index_(0), 0 );
        assert_eq!( list._unweight_index_(1), 0 );
        assert_eq!( list._unweight_index_(2), 1 );
        assert_eq!( list._unweight_index_(3), 1 );
        assert_eq!( list._unweight_index_(4), 1 );
        assert_eq!( list._unweight_index_(5), 2 );
        assert_eq!( list._unweight_index_(6), 2 );
        assert_eq!( list._unweight_index_(7), 2 );
        assert_eq!( list._unweight_index_(8), 2 );
        assert_eq!( list._unweight_index_(9), 2 );
    }

    #[test] #[should_panic] fn _unweight_index_empty_()
    {
        el()._unweight_index_(0);
    }

    #[test] #[should_panic] fn _unweight_index_out_of_bounds_()
    {
        wl()._unweight_index_(10);
    }

    #[test] fn _unweight_index_nopanic_()
    {
        let list = wl();
        assert_eq!( list._unweight_index_nopanic_(10), 3 );
        assert_eq!( list._unweight_index_nopanic_(11), 3 );
        assert_eq!( list._unweight_index_nopanic_(12), 3 );
    }

    #[test] fn _unweight_index_skipping_()
    {
        let list = wl();
        
        let seen = std::collections::HashSet::from(["sup".to_string()]);
        assert_eq!( list._unweight_index_skipping_(0, &seen), Some(1) );
        assert_eq!( list._unweight_index_skipping_(1, &seen), Some(1) );
        assert_eq!( list._unweight_index_skipping_(2, &seen), Some(1) );
        assert_eq!( list._unweight_index_skipping_(3, &seen), Some(2) );
        assert_eq!( list._unweight_index_skipping_(4, &seen), Some(2) );
        assert_eq!( list._unweight_index_skipping_(5, &seen), Some(2) );
        assert_eq!( list._unweight_index_skipping_(6, &seen), Some(2) );
        assert_eq!( list._unweight_index_skipping_(7, &seen), Some(2) );

        let seen = std::collections::HashSet::from(["nova".to_string()]);
        assert_eq!( list._unweight_index_skipping_(0, &seen), Some(0) );
        assert_eq!( list._unweight_index_skipping_(1, &seen), Some(0) );
        assert_eq!( list._unweight_index_skipping_(2, &seen), Some(2) );
        assert_eq!( list._unweight_index_skipping_(3, &seen), Some(2) );
        assert_eq!( list._unweight_index_skipping_(4, &seen), Some(2) );
        assert_eq!( list._unweight_index_skipping_(5, &seen), Some(2) );
        assert_eq!( list._unweight_index_skipping_(6, &seen), Some(2) );
    }
}
