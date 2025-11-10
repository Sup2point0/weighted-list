use num_traits::Num;


#[derive(Debug)]
pub struct WeightedItem<V,W: Num>
{
    pub weight: W,
    pub value: V,
}

impl<V,W: Num> WeightedItem<V,W>
{
    pub fn unit(value: V) -> WeightedItem<V,W>
    {
        Self {
            weight: W::one(),
            value: value
        }
    }

    pub fn new(value: V, weight: W) -> WeightedItem<V,W>
    {
        Self {
            weight,
            value,
        }
    }
}

impl<V: Eq, W: Num> Eq for WeightedItem<V,W> {}

impl<V: PartialEq, W: Num> PartialEq for WeightedItem<V,W>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.value == other.value && self.weight == other.weight
    }
}


pub struct WeightedList<V,W: Num>
{
    _data_: Vec<WeightedItem<V,W>>,
}

impl<V,W: Num> WeightedList<V,W>
{
    /// Initialise an empty `WeightedList`.
    pub fn empty() -> Self
    {
        Self {
            _data_: Vec::new()
        }
    }

    pub fn from<I>(items: I) -> Self
    where I: IntoIterator<Item = (W, V)>
    {
        Self {
            _data_: items.into_iter().map(
                |(weight, value)|
                WeightedItem::new(value, weight)
            ).collect::<Vec<WeightedItem<V,W>>>()
        }
    }

    pub fn new(
        items: Vec<(W, V)>
    ) -> Self
    {
        Self {
            _data_: items.into_iter().map(
                |(weight, value)|
                WeightedItem::new(value, weight)
            ).collect::<Vec<WeightedItem<V,W>>>()
        }
    }
}

impl<V: Eq, W: Num> Eq for WeightedList<V, W> {}

impl<V: PartialEq, W: Num> PartialEq for WeightedList<V, W>
{
    fn eq(&self, other: &Self) -> bool
    {
        self._data_.len() == other._data_.len()
    }
}
