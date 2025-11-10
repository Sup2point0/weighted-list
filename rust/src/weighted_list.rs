use num_traits::Num;


#[derive(Debug)]
struct WeightedItem<V, W: Num>
{
    pub weight: W,
    pub value: V,
}

impl<V, W: Num> WeightedItem<V, W>
{
    fn unit(value: V) -> WeightedItem<V, W>
    {
        Self {
            weight: W::one(),
            value: value
        }
    }

    fn new(value: V, weight: W) -> WeightedItem<V, W>
    {
        Self {
            weight,
            value,
        }
    }
}


struct WeightedList<V, W: Num>
{
    _data_: Vec<WeightedItem<V, W>>,
}

impl<V, W: Num> WeightedList<V, W>
{}
