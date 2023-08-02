namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;
    
    public interface ISerializableArray<T> : ISerializableObject
    {
        #region Properties
        public T[] Array { get; }
        #endregion
    }
}
