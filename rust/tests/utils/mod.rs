mod constructors;
#[allow(unused_imports)]  // FIXME why?
pub(crate) use constructors::*;

pub mod stats;

use weighted_list::*;


pub type WL = WeightedList<String, u32>;
pub type WI = WeightedItem<String, u32>;


#[macro_export]
macro_rules! str {
    ( $( $s: expr )? ) => {
        String::from( $( $s )* )
    };
}
