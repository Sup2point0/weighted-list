use crate::*;
use weighted_list::*;


#[test] fn index()
{
    let list = wl();

    assert_eq!( list[0].value, "sup" );
    assert_eq!( list[1].value, "sup" );
    assert_eq!( list[2].value, "nova" );
    assert_eq!( list[3].value, "nova" );
    assert_eq!( list[4].value, "nova" );
    assert_eq!( list[5].value, "shard" );
    assert_eq!( list[6].value, "shard" );
    assert_eq!( list[7].value, "shard" );
    assert_eq!( list[8].value, "shard" );
    assert_eq!( list[9].value, "shard" );
}

#[test] #[should_panic] fn index_empty()
{
    let list = el();
    let _ = &list[0];
}

#[test] #[should_panic] fn index_out_of_bounds()
{
    let list = wl();
    let _ = &list[10];
}

#[test] fn index_large()
{
    let list = wll();

    assert_eq!( list[0].value, "sup" );
    assert_eq!( list[1].value, "sup" );

    assert_eq!( list[2].value, "nova" );
    assert_eq!( list[3].value, "nova" );
    assert_eq!( list[4].value, "nova" );

    assert_eq!( list[5].value, "shard" );
    assert_eq!( list[6].value, "shard" );
    assert_eq!( list[7].value, "shard" );
    assert_eq!( list[8].value, "shard" );
    assert_eq!( list[9].value, "shard" );

    assert_eq!( list[10].value, "cortex" );
    assert_eq!( list[11].value, "cortex" );
    assert_eq!( list[12].value, "cortex" );
    assert_eq!( list[13].value, "cortex" );
    assert_eq!( list[14].value, "cortex" );
    assert_eq!( list[15].value, "cortex" );
    assert_eq!( list[16].value, "cortex" );

    assert_eq!( list[17].value, "origin" );
    assert_eq!( list[18].value, "origin" );
    assert_eq!( list[19].value, "origin" );
    assert_eq!( list[20].value, "origin" );
    assert_eq!( list[21].value, "origin" );
    assert_eq!( list[22].value, "origin" );
    assert_eq!( list[23].value, "origin" );
    assert_eq!( list[24].value, "origin" );
    assert_eq!( list[25].value, "origin" );
    assert_eq!( list[26].value, "origin" );
    assert_eq!( list[27].value, "origin" );
    assert_eq!( list[28].value, "origin" );
    assert_eq!( list[29].value, "origin" );
    
    assert_eq!( list[30].value, "vision" );
    assert_eq!( list[31].value, "vision" );
    assert_eq!( list[32].value, "vision" );
    assert_eq!( list[33].value, "vision" );
    assert_eq!( list[34].value, "vision" );
    assert_eq!( list[35].value, "vision" );
    assert_eq!( list[36].value, "vision" );
    assert_eq!( list[37].value, "vision" );
    assert_eq!( list[38].value, "vision" );
    assert_eq!( list[39].value, "vision" );
    assert_eq!( list[40].value, "vision" );
    assert_eq!( list[41].value, "vision" );
    assert_eq!( list[42].value, "vision" );
    assert_eq!( list[43].value, "vision" );
    assert_eq!( list[44].value, "vision" );
    assert_eq!( list[45].value, "vision" );
    assert_eq!( list[46].value, "vision" );
    assert_eq!( list[47].value, "vision" );
    assert_eq!( list[48].value, "vision" );
    assert_eq!( list[49].value, "vision" );
}


#[test] fn index_float()
{
    let list = wlist![
        (2.0, str!("sup")),
        (0.5, str!("half")),
        (7.5, str!("triple")),
    ];
    for i in 0..20   { assert_eq!( list[i as f64 / 10.0].value, "sup" ); }
    for i in 20..25  { assert_eq!( list[i as f64 / 10.0].value, "half" ); }
    for i in 25..100 { assert_eq!( list[i as f64 / 10.0].value, "triple" ); }

    let list = wlist![
        (0.2, "sups"),
        (0.3, "novae"),
        (0.5, "shards"),
    ];
    for i in 0..2  { assert_eq!( list[i as f64 / 10.0].value, "sups" ); }
    for i in 2..5  { assert_eq!( list[i as f64 / 10.0].value, "novae" ); }
    for i in 5..10 { assert_eq!( list[i as f64 / 10.0].value, "shards" ); }
}

#[test] fn index_float_extremes()
{
    let list = wlist![
        (1_000_000.0, "mega"),
        (1_000_000_000.0, "giga"),
        (1_000_000_000_000.0, "tera"),
    ];
    for i in (0..10_000_000).step_by(142857) {
        assert_eq!( list[i as f64 / 10.0].value, "mega" );
    }
    for i in (10_000_000..10_000_000_000 as i64).step_by(142857) {
        assert_eq!( list[i as f64 / 10.0].value, "giga" );
    }
    for i in (10_000_000_000..10_000_000_000 as i64).step_by(142857) {
        assert_eq!( list[i as f64 / 10.0].value, "tera" );
    }
}
