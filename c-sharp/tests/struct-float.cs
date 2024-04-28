namespace tests;


public struct tStruct
{
    public string tValue;

    public tStruct(string sup)
        => tValue = sup;
}


[TestClass]
public class tStructFloat
{
    public WeightedList<tStruct, float> test = new();

    public (float, tStruct)[] dataArray = [
        (2.0f, new tStruct("sup")),
        (3.0f, new tStruct("nova"))
    ];
    public List<(float, tStruct)> dataList = new() {
        (2.0f, new tStruct("sup")),
        (3.0f, new tStruct("nova"))
    };
    public Dictionary<tStruct, float> dataDict = new() {
        [new tStruct("sup")] = 2.0f,
        [new tStruct("nova")] = 3.0f
    };

    public void Reset(bool fill = false)
    {
        if (fill) {
            test = new WeightedList<tStruct, float>(
                (2.0f, new tStruct("sup")),
                (3.0f, new tStruct("nova"))
            );
        } else {
            test = new();
        }
    }

    [TestMethod]
    public void Indexing()
    {
        Reset(fill: true);

        WeightedItem<tStruct, float> it;

        it = test[0.5f];
        Assert.IsTrue(it == new WeightedItem<tStruct, float>(
            new tStruct("sup"), 2.0f));
        Assert.IsTrue(it.Value.Equals(new tStruct("sup")));
        Assert.IsTrue(it.Weight == 2.0f);
    }
}
