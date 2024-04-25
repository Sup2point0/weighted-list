/* frozen-weighted-list
 * v1.0.0
 * by Sup#2.0 (@Sup2point0)
 * Last updated: 18 April 2024
 * Available on GitHub: <https://github.com/Sup2point0/weightedlist>
 */

using System.Collections;
using System.Numerics;


/// <summary>
/// Represents an immutable weighted item.
/// </summary>
/// <typeparam name="V">The type of the item’s value. Can be any type.</typeparam>
/// <typeparam name="W">The type of the item’s weight. Must be a numerical type implementing<c>INumber&lt;&gt;</c>.</typeparam>
public readonly struct FrozenWeightedItem<V,W>
{
    public required V Value { get; init; }
    public required W Weight { get; init; }
    public required W cWeight { get; init; }

    public void Deconstruct(out V value, out W weight)
    {
        value = Value;
        weight = Weight;
    }

    public override string ToString()
        => base.ToString() + $"{{value: {Value}, weight: {Weight}, collective weight: {cWeight}}}";

    public GetHashCode()
        => base.GetHashCode();

    public static bool operator ==(FrozenWeightedItem<V,W> item1, FrozenWeightedItem<V,W> item2)
        => item1.Equals(item2);

    public static bool operator !=(FrozenWeightedItem<V,W> item1, FrozenWeightedItem<V,W> item2)
        => !item1.Equals(item2);

    public bool Equals(object obj)
        => (obj is FrozenWeightedItem<V,W>)
            ? obj.Value == this.Value && obj.Weight == this.Weight
            : false;
}


/// <summary>
/// Represents an immutable list of weighted items.
/// </summary>
/// <typeparam name="V">The type of the items’ values. Can be any type.</typeparam>
/// <typeparam name="W">The type of the items’ weights. Must be a numerical type implementing <c>INumber&lt;&gt;</c>.</typeparam>
class FrozenWeightedList<V,W> :
    IEnumerable<FrozenWeightedItem<V,W>>
    where W : INumber<W>
{
    #region FIELDS

    public const string VERSION = "1.0.0";

    private FrozenWeightedItem<V,W>[] _data;

    public readonly int TotalValues;
    public readonly W TotalWeights;

    #endregion

    #region CONSTRUCTORS

    /// Root constructor method called by all constructors. Calculates the constant fields of the list since it won’t be modified.
    private Construct((W weight, V value)[] items)
    {
        TotalValues = items.Count;
        TotalWeights = W.Zero;

        _data = new FrozenWeightedItem<V,W>[TotalValues];

        for (int i = 0; i < TotalValues; i++) {
            _data[i] = new FrozenWeightedItem() {
                Value = items[i].value,
                Weight = items[i].weight,
                cWeight = TotalWeights,
            }
            TotalWeights += items[i].weight;
        }
    }

    public FrozenWeightedList(params[] (W weight, V value) items)
        => Construct(items);

    public FrozenWeightedList(IEnumerable<(W weight, V value)> items)
        => Construct(items);

    public FrozenWeightedList(Dictionary<V,W> items)
        => Construct(from each in items select (each.Value, each.Key).ToArray());

    #endregion

    #region INTERNAL

    protected FrozenWeightedItem<V,W> _FindAtWeightedIndex(W index)
    {
        if (index >= TotalWeights) {
            throw new IndexOutOfRangeException(
                $"Attempted to access index {index} but FrozenWeightedList is only {TotalWeights - W.One} long");
        }

        FrozenWeightedItem<V,W> item;
        double idx = TotalValues / 2.0d;
        double step = idx;

        for (int i = 1; i < TotalWeights; i++) {
            step /= 2;
            item = _data[Math.Round(idx)];

            if (
                item.cWeight + item.Weight - W.One >= index
                && index >= item.cWeight
            ) {
                return item;
            } else if (index > item.cWeight) {
                idx += step;
            } else if (index < item.cWeight) {
                idx -= step;
            } else {
                throw new IndexOutOfRangeException(
                    $"Attempted to shift index {idx} but an unknown error occurred");
            }
        }

        throw new IndexOutOfRangeException(
            $"Failed to find item at index {idx}");
    }

    #endregion

    #region INTERFACES

    public FrozenWeightedItem<V,W> this[W index] {
        get => _FindAtWeightedIndex(index);
    }

    public V GetValueAt(W index, bool weighted = true)
        => GetItemAt(index, weighted).Value;

    public FrozenWeightedItem<V,W> GetItemAt(W index, bool weighted = true)
        => weighted ? _FindAtWeightedIndex(index) : _data[int.CreateChecked(index)];

    public override string ToString()
        => "FrozenWeightedList<> {\n\t" + string.Join("\n\t", from each in _data select each.ToString()) + "\n}";

    public override int GetHashCode()
        => base.GetHashCode();

    #endregion

    #region SPECIALIST METHODS

    public V GetRandomValue()
        => GetRandomItem().Value;

    public V[] GetRandomValues(int count, bool unique = false, bool replace = true)
        => (from each in GetRandomItems(count, unique, replace) select each.Value).ToArray();

    public WeightedItem<V,W> GetRandomItem()
    {
        double index;
        index = new Random().NextDouble();
        index *= double.CreateChecked(TotalWeights);
        index = Math.Floor(index);

        return _FindAtWeightedIndex(W.CreateChecked(index));
    }

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
            return data;
        } else {
            return new HashSet<V>(data).ToArray();
        }
    }

    public W[] GetWeights()
        => (from each in _data select each.Weight).ToArray();

    public Dictionary<V,W> ToDictionary()
        => _data.ToDictionary(
            item => item.Value,
            item => item.Weight
        );

    #endregion
}
