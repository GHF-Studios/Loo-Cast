using System;

namespace LooCast.System.Serialization
{
    public interface ISerializer
    {
        #region Properties
        SerializationType SerializationType { get; }
        Type SerializableType { get; }
        Type SerializedType { get; }
        #endregion
    }
}
