namespace tests;


[TestClass]
public class tStringInt
{
    public WeightedList<string, int> test = new();

    public (int, string)[] dataArray = [
        (2, "sup"), (3, "nova") ];
    public List<(int, string)> dataList = new() {
        (2, "sup"), (3, "nova") };
    public Dictionary<string, int> dataDict = new() {
        ["sup"] = 2, ["nova"] = 3 };

    public void Reset(bool fill = false)
        => test = fill ? new WeightedList<string, int>((2, "sup"), (3, "nova")) : new();

    // CORE //
    [TestMethod]
    public void Constructors()
    {
        // weight types
        WeightedList<string, int> t1 = new();
        WeightedList<string, float> t2 = new();
        WeightedList<string, double> t3 = new();

        // data unpacking
        WeightedList<string, int> expect = new((2, "sup"), (3, "nova"));

        Reset(fill: true);
        test = new(
            new WeightedItem<string, int>("sup", 2),
            new WeightedItem<string, int>("nova", 3)
        );
        Assert.IsTrue(test == expect);

        Reset(fill: true);
        test = new(dataArray);
        Assert.IsTrue(test == expect);

        Reset(fill: true);
        test = new(dataList);
        Assert.IsTrue(test == expect);
        
        Reset(fill: true);
        test = new(dataDict);
        Assert.IsTrue(test == expect);
    }

    [TestMethod]
    public void Deconstructor()
    {
        Reset(fill: true);
        (string value, int weight) = test[0];
        Assert.IsTrue(value == "sup");
        Assert.IsTrue(weight == 2);
    }

    // STATIC //
    [TestMethod]
    public void Properties()
    {
        Reset(fill: true);

        Assert.IsTrue(test.TotalValues == 2);
        Assert.IsTrue(test.TotalWeights == 5);

        test.Clear();
        Assert.IsTrue(test.TotalValues == 0);
        Assert.IsTrue(test.TotalWeights == 0);
    }

    [TestMethod]
    public void Indexing()
    {
        Reset(fill: true);

        WeightedItem<string, int> it;

        it = test[0];
        Assert.IsTrue(it == new WeightedItem<string, int>("sup", 2));
        Assert.IsTrue(it.Value == "sup");
        Assert.IsTrue(it.Weight == 2);

        it = test[3];
        Assert.IsTrue(it == new WeightedItem<string, int>("nova", 3));
        Assert.IsTrue(it.Value == "nova");
        Assert.IsTrue(it.Weight == 3);
    }

    [TestMethod]
    public void Entirety()
    {
        WeightedList<string, int> expect;

        Reset(fill: true);
        test.Clear();
        expect = new();
        Assert.IsTrue(test == expect);

        Reset(fill: true);
        expect = new((3, "nova"), (2, "sup"));
        Assert.IsTrue(test.GetReversed() == expect);
        Assert.IsTrue(test.Reverse() == expect);
    }

    // DYNAMIC //
    [TestMethod]
    public void Copying()
    {
        Reset(fill: true);
        var copy = test.DeepClone();
        copy.Clear();
        Assert.IsTrue(test.TotalValues > 0);
    }

    [TestMethod]
    public void Mutating()
    {
        WeightedList<string, int> expect;

        Reset();

        test.AddValue("sup");
        expect = new((1, "sup"));
        Assert.IsTrue(test == expect);

        test.RemoveAt(0);
        expect = new();
        Assert.IsTrue(test == expect);

        // expect = new(dataList);
        // {
        //     Reset();
        //     test.AddItemRange(dataArray.AsEnumerable());
        //     Assert.IsTrue(test == expect);

        //     Reset();
        //     test.AddItemRange(dataList.AsEnumerable());
        //     Assert.IsTrue(test == expect);

        //     Reset();
        //     test.AddItemRange(from each in dataList select each);
        //     Assert.IsTrue(test == expect);
        // }
    }

    [TestMethod]
    public void Searches()
    {
        Reset(fill: true);

        Assert.IsTrue(test.GetIndexOfValue("nova") == 2);
        Assert.IsTrue(test.GetIndexOfValue("nova", 3) == 3);
        Assert.IsTrue(test.GetIndexOfValue("nova", 0, 2) == -1);
    }

    [TestMethod]
    public void PredicateSearches()
    {
        Predicate<WeightedItem<string, int>> tV = item => item.Value == "sup";
        Predicate<WeightedItem<string, int>> fV = item => item.Value == "soup";
        Predicate<WeightedItem<string, int>> tW = item => item.Weight > 2;
        Predicate<WeightedItem<string, int>> fW = item => item.Weight < 2;

        Reset(fill: true);

        Assert.IsTrue(test.Exists(tV));
        Assert.IsFalse(test.Exists(fW));

        Assert.IsTrue(test.TrueForAll(x => x.Weight > 0));

        Assert.IsTrue(test.FindIndex(tV) == 0);
        Assert.IsTrue(test.FindIndex(tW) == 2);

        Assert.IsTrue(test.FindIndex(2, tV) == -1);
        Assert.IsTrue(test.FindIndex(3, tW) == 3);

        Assert.IsTrue(test.FindIndex(0, 3, tW) == 2);
        Assert.IsTrue(test.FindIndex(0, 2, tW) == -1);
    }

    [TestMethod]
    public void Data()
    {
        Reset(fill: true);
        {
            string[] expect = ["sup", "sup", "nova", "nova", "nova"];
            Assert.IsTrue(expect.SequenceEqual(test.GetRaw()));
        }
        {
            string[] expect = ["sup", "nova"];
            Assert.IsTrue(expect.SequenceEqual(test.GetValues()));
        }
        {
            int[] expect = [2, 3];
            Assert.IsTrue(expect.SequenceEqual(test.GetWeights()));
        }
        {
            // WeightedItem<string, int>[] expect = [
            //     new WeightedItem<string, int>("sup", 2),
            //     new WeightedItem<string, int>("nova", 3)
            // ];
            // Assert.IsTrue(expect.SequenceEqual(test.ToArray()));
        }
        {
            Dictionary<string, int> expect = new() {
                ["sup"] = 2, ["nova"] = 3
            };
            Assert.IsTrue(expect.SequenceEqual(test.ToDictionary()));
        }
    }
}
