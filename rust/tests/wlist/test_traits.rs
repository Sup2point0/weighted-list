use crate::*;
// use weighted_list::*;


#[test] fn clone() {
    let list = wl();
    let mut cloned = list.clone();
    cloned.clear();
    assert_ne!( list, cloned );
}

#[test] fn asref_vec()
{
    let list = wl();

    fn test<T>(_: impl AsRef<Vec<T>>) {}

    test(list);
}

#[test] fn asref_slice()
{
    let list = wl();

    fn test<T>(_: impl AsRef<[T]>) {}

    test(list);
}

#[test] fn asmut_vec()
{
    let list = wl();

    fn test<T>(_: impl AsMut<Vec<T>>) {}

    test(list);
}

#[test] fn asmut_slice()
{
    let list = wl();

    fn test<T>(_: impl AsMut<[T]>) {}

    test(list);
}

#[test] fn deref_to_slice()
{
    let list = wl();

    let _ = list.first();
    let _ = (*list).len();
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
