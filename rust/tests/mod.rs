mod utils;
pub use utils::*;


mod wlist {
    mod test_all;

    mod test_constructors;

    mod test_accessors;

    mod test_properties;

    mod test_equality;

    mod test_conversions;

    mod test_slice;

    mod test_traits;

    mod test_index;

    mod test_list_mut;

    mod test_wlist_mut;

    mod test_random;
    
    mod test_random_stats;
}


mod fwlist
{
    mod test_constructors;

    mod test_index;
}
