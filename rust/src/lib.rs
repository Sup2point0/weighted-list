//! A list implementation for weighted randomisation.
//! 
//! See `WeightedList` for more.

mod root;


mod weighted_item;
pub use weighted_item::{WeightedItem, WItem};

mod weighted_list;
pub use weighted_list::{WeightedList, WList};
