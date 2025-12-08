use crate::*;
// use weighted_list::*;


#[test] fn index()
{
    let list = fwl();
    assert_eq!( list[0].value(), "sup" );
    assert_eq!( list[1].value(), "sup" );
    assert_eq!( list[2].value(), "nova" );
    assert_eq!( list[3].value(), "nova" );
    assert_eq!( list[4].value(), "nova" );
    assert_eq!( list[5].value(), "shard" );
    assert_eq!( list[6].value(), "shard" );
    assert_eq!( list[7].value(), "shard" );
    assert_eq!( list[8].value(), "shard" );
    assert_eq!( list[9].value(), "shard" );
}

#[test] #[should_panic] fn index_empty()
{
    let list = efl();
    let _ = &list[0];
}

#[test] #[should_panic] fn index_out_of_bounds()
{
    let list = fwl();
    let _ = &list[10];
}

#[test] fn iter_methods()
{
    let list = wl();

    for _ in list.iter() {}
}

#[test] fn iter_sugar()
{
    let list = fwl();

    for _ in &list {}

    for _ in list {}
}

#[test] fn test_index_large()
{
    let list = fwll();
    assert_eq!( list[0].value(),  "sup"    );
    assert_eq!( list[1].value(),  "sup"    );
    assert_eq!( list[2].value(),  "nova"   );
    assert_eq!( list[3].value(),  "nova"   );
    assert_eq!( list[4].value(),  "nova"   );
    assert_eq!( list[5].value(),  "shard"  );
    assert_eq!( list[6].value(),  "shard"  );
    assert_eq!( list[7].value(),  "shard"  );
    assert_eq!( list[8].value(),  "shard"  );
    assert_eq!( list[9].value(),  "shard"  );
    assert_eq!( list[10].value(), "cortex" );
    assert_eq!( list[11].value(), "cortex" );
    assert_eq!( list[12].value(), "cortex" );
    assert_eq!( list[13].value(), "cortex" );
    assert_eq!( list[14].value(), "cortex" );
    assert_eq!( list[15].value(), "cortex" );
    assert_eq!( list[16].value(), "cortex" );
    assert_eq!( list[17].value(), "origin" );
    assert_eq!( list[18].value(), "origin" );
    assert_eq!( list[19].value(), "origin" );
    assert_eq!( list[20].value(), "origin" );
    assert_eq!( list[21].value(), "origin" );
    assert_eq!( list[22].value(), "origin" );
    assert_eq!( list[23].value(), "origin" );
    assert_eq!( list[24].value(), "origin" );
    assert_eq!( list[25].value(), "origin" );
    assert_eq!( list[26].value(), "origin" );
    assert_eq!( list[27].value(), "origin" );
    assert_eq!( list[28].value(), "origin" );
    assert_eq!( list[29].value(), "origin" );
    assert_eq!( list[30].value(), "vision" );
    assert_eq!( list[31].value(), "vision" );
    assert_eq!( list[32].value(), "vision" );
    assert_eq!( list[33].value(), "vision" );
    assert_eq!( list[34].value(), "vision" );
    assert_eq!( list[35].value(), "vision" );
    assert_eq!( list[36].value(), "vision" );
    assert_eq!( list[37].value(), "vision" );
    assert_eq!( list[38].value(), "vision" );
    assert_eq!( list[39].value(), "vision" );
    assert_eq!( list[40].value(), "vision" );
    assert_eq!( list[41].value(), "vision" );
    assert_eq!( list[42].value(), "vision" );
    assert_eq!( list[43].value(), "vision" );
    assert_eq!( list[44].value(), "vision" );
    assert_eq!( list[45].value(), "vision" );
    assert_eq!( list[46].value(), "vision" );
    assert_eq!( list[47].value(), "vision" );
    assert_eq!( list[48].value(), "vision" );
    assert_eq!( list[49].value(), "vision" );
}
