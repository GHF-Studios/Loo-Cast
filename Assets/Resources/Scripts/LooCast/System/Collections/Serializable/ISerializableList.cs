using System.Collections;
using System.Collections.Generic;

namespace LooCast.System.Collections.Serializable
{
    public interface ISerializableList : IList
    {
    }
    
    public interface ISerializableList<T> : ISerializableList, IList<T>
    {
    }
}
