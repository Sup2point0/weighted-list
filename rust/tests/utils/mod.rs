mod constructors;
#[allow(unused_imports)]  // FIXME why?
pub(crate) use constructors::*;

pub mod stats;


#[macro_export]
macro_rules! str {
    ( $( $s: expr )? ) => {
        String::from( $( $s )* )
    };
}
