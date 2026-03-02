//! Data structures for weighted randomisation.
//! 
//! This crate provides the [`WeightedList<V,W>`](WeightedList) struct, storing [`WeightedItem`]s with a value and weight. When picking items randomly, those with a higher weight are more likely to be chosen.
//! 
//! ## Example
//! 
//! ```
//! use weighted_list::*;
//! 
//! let wl = WeightedList::<String, u8>::from([
//!     (2, "sup".to_string()),
//!     (3, "nova".to_string()),
//!     (5, "shard".to_string()),
//! ]);
//! 
//! for item in &wl {
//!     println!("{} has weight {}", item.value, item.weight);
//! }
//! 
//! if let Some(result) = wl.select_random_value(&mut rand::rng()) {
//!     println!("{result}");
//! }
//! ```
//! 
//! ## Why might you need this?
//! 
//! - Weighted randomisation for a reward system
//! - Item stacking for an inventory system
//! - Statistical sampling
//! 
//! For more detailed guidance on how to use the struct, see [`WeightedList`].

mod weighted_item;
pub use weighted_item::{ WeightedItem, WItem };

mod weighted_list;
pub use weighted_list::{ WeightedList, WList };

mod traits;
pub use traits::{ Weight };

pub mod errors;

pub(crate) mod util;


#[cfg(feature = "frozen")] mod frozen_weighted_item;
#[cfg(feature = "frozen")] pub use frozen_weighted_item::{ FrozenWeightedItem, FWItem };

#[cfg(feature = "frozen")] mod frozen_weighted_list;
#[cfg(feature = "frozen")] pub use frozen_weighted_list::{ FrozenWeightedList, FWList };
