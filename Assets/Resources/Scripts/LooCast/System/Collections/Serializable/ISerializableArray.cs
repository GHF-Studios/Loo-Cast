namespace LooCast.System.Collections.Serializable
{
    public interface ISerializableArray
    {
        int Length { get; }
        object this[int index] { get; set; }
    }

    public interface ISerializableArray<T> : ISerializableArray
    {
        #region Properties
        T[] Array { get; }
        #endregion
    }
}
