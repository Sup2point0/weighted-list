use crate::*;
// use weighted_list::*;


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
