namespace LooCast.Data
{
    public interface IDataProvider<DataType> where DataType : IData
    {
        DataType PersistentData { get; set; }
    }
}
