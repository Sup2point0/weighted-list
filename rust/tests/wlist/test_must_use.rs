use crate::*;


#[test] fn out_of_place()
{
    let list = wl();

    /* NOTE: These should emit `must_use` compiler warnings */
    list.pruned();
    list.reversed();
}
