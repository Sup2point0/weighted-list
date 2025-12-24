use crate::*;
// use weighted_list::*;


#[test] fn clone() {
    let list = wl();
    let mut cloned = list.clone();
    cloned.clear();
    assert_ne!( list, cloned );
}


#[test] fn printing()
{
    println!("\n");

    let list = el();
    println!("--- el.debug == {list:?}\n");
    println!("--- el.display == {list}\n");

    let list = wl();
    println!("--- wl.debug == {list:?}\n");
    println!("--- wl.display == {list}\n");
}
