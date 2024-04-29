/// WeightedList
/// v1.0.3
/// by Sup#2.0 (@Sup2point0)
/// Last updated: 18 April 2024
/// Available on GitHub: <https://github.com/Sup2point0/weightedlist>

using System.Collections;
using System.Numerics;


/// <summary>
/// Represents a weighted item.
/// </summary>
/// <typeparam name="V">The type of the item’s value. Can be any type.</typeparam>
/// <typeparam name="W">The type of the item’s weight. Must be a numerical type implementing<c>INumber&lt;&gt;</c>.</typeparam>
public class WeightedItem<V,W> where W : INumber<W>
{
    #region FIELDS

    public V Value;
    public W Weight;

    #endregion

    #region CONSTRUCTORS

    public WeightedItem(V value)
    {
        this.Value = value;
        this.Weight = W.One;
    }

    public WeightedItem(V value, W weight)
    {
        this.Value = value;
        this.Weight = weight;
    }

    public WeightedItem((W weight, V value) item)
    {
        this.Value = item.value;
        this.Weight = item.weight;
    }

    public WeightedItem(KeyValuePair<V,W> item)
    {
        this.Value = item.Key;
        this.Weight = item.Value;
    }

    #endregion

    #region INTERFACES

    public void Deconstruct(out V value, out W weight)
    {
        value = Value;
        weight = Weight;
    }

    public WeightedItem<V,W> Clone()
        => new WeightedItem<V,W>(Value, Weight);

    public override string ToString()
        => base.ToString() + $"{{value: {Value}, weight: {Weight}}}";

    public override int GetHashCode()
        => base.GetHashCode();

    #endregion

    #region OPERATORS

    public static bool operator ==(WeightedItem<V,W> item1, WeightedItem<V,W> item2)
        => item1.Equals(item2);

    public static bool operator !=(WeightedItem<V,W> item1, WeightedItem<V,W> item2)
        => !item1.Equals(item2);

    public override bool Equals(object obj)
        => (obj is WeightedItem<V,W>) ? Equals(obj) : false;

    public bool Equals(WeightedItem<V,W> item)
        => this.Value.Equals(item.Value) && this.Weight == item.Weight;

    #endregion

    #region DATA METHODS

    public (W, V) ToTuple()
        => (Weight, Value);
    
    #endregion

}


/// <summary>
/// Represents a list of weighted items.
/// </summary>
/// <typeparam name="V">The type of the items’ values. Can be any type.</typeparam>
/// <typeparam name="W">The type of the items’ weights. Must be a numerical type implementing <c>INumber&lt;&gt;</c>.</typeparam>
public class WeightedList<V,W> :
    IEnumerable<WeightedItem<V,W>>
    where W : INumber<W>
{
    #region FIELDS

    public const string VERSION = "1.0.0";

    private List<WeightedItem<V,W>> _data;

    #endregion

    #region CONSTRUCTORS

    public WeightedList()
        => _data = new();

    public WeightedList(params WeightedItem<V,W>[] items)
        => _data = new List<WeightedItem<V,W>>(items);

    public WeightedList(params (W weight, V value)[] items)
        => _data = new List<WeightedItem<V,W>>(
            from each in items select new WeightedItem<V,W>(each));

    public WeightedList(params V[] values)
        => _data = new List<WeightedItem<V,W>>(
            from each in values select new WeightedItem<V,W>(each));

    public WeightedList(IEnumerable<WeightedItem<V,W>> items)
        => _data = new List<WeightedItem<V,W>>(items);

    public WeightedList(IEnumerable<(W weight, V value)> items)
        => _data = new List<WeightedItem<V,W>>(
            from each in items select new WeightedItem<V,W>(each));

    public WeightedList(IEnumerable<V> values)
        => _data = new List<WeightedItem<V,W>>(
            from each in values select new WeightedItem<V,W>(each));

    public WeightedList(Dictionary<V,W> items)
    {
        foreach (KeyValuePair<V,W> item in items) {
            _data.Add(new WeightedItem<V,W>(item));
        }
    }

    #endregion

    #region PROPERTIES

    public int TotalValues {
        get => _data.Count; }

    public W TotalWeights {
        get {
            W t = W.Zero;
            foreach (var item in _data) {
                t += item.Weight;
            }
            return t;
        }
    }

    #endregion

    #region INTERNAL

    protected int _UnweightIndex(W index)
    {
        W total = W.Zero;
        int i = 0;

        foreach (var item in _data) {
            total += item.Weight;
            if (total > index) {
                return i;
            }
            i++;
        }

        throw new IndexOutOfRangeException(
            $"Attempted to access index {index} but WeightedList is only {TotalWeights} long"
        );
    }

    protected WeightedItem<V,W> _FindAtWeightedIndex(W index, WeightedItem<V,W> replace = null)
    {
        W t = W.Zero;

        foreach (var item in _data) {
            t += item.Weight;
            if (t > index) {
                if (replace is not null) {
                    item.Value = replace.Value;
                    item.Weight = replace.Weight;
                }
                return item;
            }
        }

        throw new IndexOutOfRangeException(
            $"Attempted to access index {index} but WeightedList is only {TotalWeights - W.Zero} long"
        );
    }

    protected WeightedItem<V,W> _FindItem(WeightedItem<V,W> target)
    {
        foreach (var item in _data) {
            if (item == target) {
                return item;
            }
        }
        return null;
    }

    #endregion

    #region INTERFACES

    public WeightedItem<V,W> this[W index]
    {
        get => _FindAtWeightedIndex(index);
        set => _FindAtWeightedIndex(index, replace: value);
    }

    public V GetValueAt(W index, bool weighted = true)
        => GetItemAt(index, weighted).Value;

    public WeightedItem<V,W> GetItemAt(W index, bool weighted = true)
    {
        if (weighted) {
            return _FindAtWeightedIndex(index);
        } else {
            return _data[int.CreateChecked(index)];
        }
    }

    public bool ContainsValue(V value)
    {
        foreach (var item in _data) {
            if (item.Value.Equals(value)) {
                return true;
            }
        }
        return false;
    }

    public bool ContainsItem(WeightedItem<V,W> target)
    {
        foreach (var item in _data) {
            if (item == target) {
                return true;
            }
        }
        return false;
    }

    public IEnumerator<WeightedItem<V,W>> GetEnumerator()
        => _data.GetEnumerator();

    // IEnumerator<WeightedItem<V,W>> IEnumerable<WeightedItem<V,W>>.GetEnumerator()
    //     => GetEnumerator();

    IEnumerator IEnumerable.GetEnumerator()
        => GetEnumerator();

    public override string ToString()
        => "WeightedList<> {\n\t" + string.Join("\n\t", from each in _data select each.ToString()) + "\n}";

    public override int GetHashCode()
        => base.GetHashCode();

    public WeightedList<V,W> DeepClone()
        => new WeightedList<V,W>(
            from item in _data select
            new WeightedItem<V,W>(item.Value, item.Weight)
        );
    
    #endregion

    #region OPERATORS

    public static bool operator ==(WeightedList<V,W> list1, WeightedList<V,W> list2)
        => list1.Equals(list2);

    public static bool operator !=(WeightedList<V,W> list1, WeightedList<V,W> list2)
        => !list1.Equals(list2);

    public override bool Equals(object obj)
        => (obj is WeightedList<V,W>) ? Equals(obj) : false;

    public bool Equals(WeightedList<V,W> obj)
        => _data.SequenceEqual(obj._data);

    #endregion

    #region LIST METHODS

    public WeightedList<V,W> AddValue(V item)
    {
        _data.Add(new WeightedItem<V,W>(item));
        return this;
    }

    public WeightedList<V,W> AddValue(V item, W weight)
    {
        _data.Add(new WeightedItem<V,W>(item, weight));
        return this;
    }

    public WeightedList<V,W> AddItem(WeightedItem<V,W> item)
    {
        _data.Add(item);
        return this;
    }

    public WeightedList<V,W> AddItem((V value, W weight) item)
    {
        _data.Add(new WeightedItem<V,W>(item.value, item.weight));
        return this;
    }

    public WeightedList<V,W> AddValueRange(IEnumerable<V> collection)
    {
        foreach (V value in collection) {
            _data.Add(new WeightedItem<V,W>(value));
        }
        return this;
    }

    public WeightedList<V,W> AddItemRange(IEnumerable<WeightedItem<V,W>> collection)
    {
        foreach (var item in collection) {
            _data.Add(item);
        }
        return this;
    }

    public WeightedList<V,W> InsertValue(W index, V value)
    {
        _data.Insert(_UnweightIndex(index), new WeightedItem<V,W>(value));
        return this;
    }

    public WeightedList<V,W> InsertItem(W index, WeightedItem<V,W> item)
    {
        _data.Insert(_UnweightIndex(index), item);
        return this;
    }

    public WeightedList<V,W> InsertValueRange(W index, IEnumerable<V> collection)
    {
        var items = from each in collection select new WeightedItem<V,W>(each);
        _data.InsertRange(_UnweightIndex(index), items);
        return this;
    }

    public WeightedList<V,W> InsertItemRange(W index, IEnumerable<WeightedItem<V,W>> collection)
    {
        _data.InsertRange(_UnweightIndex(index), collection);
        return this;
    }

    public WeightedList<V,W> ReplaceValues(V search, V replace)
    {
        foreach (var item in _data) {
            if (item.Value.Equals(search)) {
                item.Value = replace;
            }
        }
        return this;
    }

    public WeightedList<V,W> ReplaceWeights(W search, W replace)
    {
        foreach (var item in _data) {
            if (item.Weight == search) {
                item.Weight = replace;
            }
        }
        return this;
    }

    public bool RemoveValue(V value, int occurrence = 1)
    { // TODO error-check occurrence value?
        int count = 0;
        int i = 0;

        foreach (var item in _data) {
            if (item.Value.Equals(value)) {
                count++;
                if (count >= occurrence) {
                    _data.RemoveAt(i);
                    return true;
                }
            }
            i++;
        }

        return false;
    }

    public WeightedList<V,W> RemoveItem(WeightedItem<V,W> target)
    {
        _data.Remove(target);
        return this;
    }

    public void RemoveAt(W index)
        => _data.RemoveAt(_UnweightIndex(index));

    public WeightedList<V,W> RemoveAll(Predicate<WeightedItem<V,W>> match)
    {
        int idx = 0;

        for (int i = 0; i < TotalValues; i++) {
            if (match(_data[i])) {
                _data.RemoveAt(idx--);
            }
            idx++;
        }

        return this;
    }

    public WeightedItem<V,W> PopAt(W index)
    {
        int idx = _UnweightIndex(index);
        var item = _data[idx];

        if (item.Weight > W.Zero) {
            item.Weight--;
        } else {
            _data.RemoveAt(idx);
        }

        return item;
    }

    public WeightedList<V,W> Clear()
    {
        _data.Clear();
        return this;
    }

    public WeightedList<V,W> GetCleared()
        => DeepClone().Clear();

    public WeightedList<V,W> Reverse()
    {
        _data.Reverse();
        return this;
    }

    public WeightedList<V,W> GetReversed()
        => DeepClone().Reverse();

    // FIXME
    // public WeightedList<V,W> Sort()
    //     => Sort(W.Zero, TotalValues, null);

    // public WeightedList<V,W> Sort(IComparer<WeightedItem<V,W>> comparer)
    //     => Sort(W.Zero, TotalValues, comparer);

    // public WeightedList<V,W> Sort(W index, int count, IComparer<WeightedItem<V,W>> comparer)
    //     => Array.Sort<List<WeightedItem<V,W>>>(_data, _UnweightIndex(index), count, comparer);

    #endregion

    #region SEARCHES

    public bool Exists(Predicate<WeightedItem<V,W>> match)
        => FindIndex(match) != W.CreateChecked(-1);

    public bool TrueForAll(Predicate<WeightedItem<V,W>> match)
    {
        foreach (var item in _data) {
            if (!match(item)) {
                return false;
            }
        }
        return true;
    }

    public WeightedItem<V,W> Find(Predicate<WeightedItem<V,W>> match)
    {
        foreach (var item in _data) {
            if (match(item)) {
                return item;
            }
        }
        return null;
    }

    public List<WeightedItem<V,W>> FindAll(Predicate<WeightedItem<V,W>> match)
    {
        List<WeightedItem<V,W>> res = new();

        foreach (var item in _data) {
            if (match(item)) {
                res.Add(item);
            }
        }

        return res;
    }

    public W FindIndex(Predicate<WeightedItem<V,W>> match)
        => FindIndex(W.Zero, TotalWeights, match);

    public W FindIndex(W index, Predicate<WeightedItem<V,W>> match)
        => FindIndex(index, TotalWeights, match);

    public W FindIndex(W index, W count, Predicate<WeightedItem<V,W>> match)
    {
        W t = W.Zero;

        foreach (var item in _data) {
            if (index >= t) {
                continue;
            } else if (t - index > count) {
                return W.CreateChecked(-1);
            } else if (match(item)) {
                return t;
            }
            t += item.Weight;
        }

        return W.CreateChecked(-1);
    }

    public W GetIndexOfValue(V value)
        => GetIndexOfValue(value, W.Zero, TotalWeights);

    public W GetIndexOfValue(V value, W index)
        => GetIndexOfValue(value, index, TotalWeights);

    public W GetIndexOfValue(V value, W index, W count)
    {
        W t = W.Zero;

        foreach (var item in _data) {
            t += item.Weight;
            if (index >= t) {
                continue;
            } else if (t - index > count) {
                return W.CreateChecked(-1);
            } else if (item.Value.Equals(value)) {
                return t;
            }
        }

        return W.CreateChecked(-1);
    }

    // SPECIALIST METHODS //
    public WeightedList<V,W> Merge(WeightedList<V,W> list)
    {
        foreach (var item in list._data) {
            var existing = _FindItem(item);
            if (existing is not null) {
                existing.Weight += item.Weight;
            } else {
                _data.Add(item);
            }
        }

        return this;
    }

    public WeightedList<V,W> GetMerged(WeightedList<V,W> list)
        => new WeightedList<V,W>(DeepClone()._data).Merge(list);

    public WeightedList<V,W> Collapse()
    {
        Dictionary<V,W> data = new();

        foreach (var item in _data) {
            (V value, W weight) = item;
            if (data.ContainsKey(value)) {
                data[value] += weight;
            } else {
                data[value] = weight;
            }
        }

        _data = (from item in data select new WeightedItem<V,W>(item)).ToList();

        return this;
    }

    public WeightedList<V,W> GetCollapsed()
        => DeepClone().Collapse();

    public V GetRandomValue()
        => GetRandomItem().Value;

    public V[] GetRandomValues(int count, bool unique = false, bool replace = true)
        => (from each in GetRandomItems(count, unique, replace) select each.Value).ToArray();

    public WeightedItem<V,W> GetRandomItem()
    {
        Random rand = new Random();

        double index;
        index = rand.NextDouble();
        index *= double.CreateChecked(TotalWeights);
        index = Math.Floor(index);

        return _FindAtWeightedIndex(W.CreateChecked(index));
    }

    public WeightedItem<V,W>[] GetRandomItems(
        int count,
        bool unique = false,
        bool replace = true
    )
    {
        if (count < 1) {
            throw new ArgumentException("Cannot get a negative number of items");
        }

        var res = new WeightedItem<V,W>[count];

        WeightedList<V,W> pool;
        if (replace) {
            pool = this;
        } else {
            // If selecting without replacement, create a copy to modify.
            pool = new(
                from item in _data select
                new WeightedItem<V,W>(item.Value, item.Weight)
            );
        }

        Random rand = new Random();
        double index;
        int i = 0;

        while (i < count) {
            index = rand.NextDouble();
            index *= double.CreateChecked(TotalWeights);
            index = Math.Floor(index);

            if (unique) {
                var candidate = pool._FindAtWeightedIndex(W.CreateChecked(index));
                if (res.Contains(candidate)) {
                    continue;
                } else {
                    res[i] = candidate;
                    i++;
                }
            } else {
                res[i] = pool._FindAtWeightedIndex(W.CreateChecked(index));
            }
        }

        return res;
    }

    public WeightedList<V,T> GetNormalised<T>() where T : INumber<T>
    {
        W total = TotalWeights;
        return new WeightedList<V,T>(
            from item in _data select
            new WeightedItem<V,T>(item.Value, T.CreateChecked(item.Weight / total))
        );
    }

    public WeightedList<V,T> GetNormalized<T>() where T : INumber<T>
        => GetNormalised<T>();

    #endregion

    #region DATA METHODS

    public V[] GetRaw()
    {
        List<V> items = new();

        foreach (var item in _data) {
            for (int i = 0; i < int.CreateChecked(item.Weight); i++) {
                items.Add(item.Value);
            }
        }

        return items.ToArray();
    }

    public V[] GetValues(bool unique = false)
    {
        V[] data = (from each in _data select each.Value).ToArray();
        if (unique) {
            return new HashSet<V>(data).ToArray();
        } else {
            return data;
        }
    }

    public W[] GetWeights()
        => (from each in _data select each.Weight).ToArray();

    public WeightedItem<V,W>[] ToArray()
        => _data.ToArray();

    public Dictionary<V,W> ToDictionary()
        => _data.ToDictionary(
            item => item.Value,
            item => item.Weight
        );

    #endregion

}
