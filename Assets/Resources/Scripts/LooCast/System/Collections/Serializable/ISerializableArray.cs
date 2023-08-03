using System.Collections;
using System.Collections.Generic;

namespace LooCast.System.Collections.Serializable
{
    public interface ISerializableArray : IEnumerable
    {
        int Length { get; }
        object this[int index] { get; set; }
    }

    public interface ISerializableArray<T> : ISerializableArray, IEnumerable<T>
    {
        #region Properties
        T[] Array { get; }
        #endregion
    }
}
