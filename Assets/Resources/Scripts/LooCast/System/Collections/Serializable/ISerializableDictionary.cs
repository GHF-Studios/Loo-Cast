using System.Collections;
using System.Collections.Generic;

namespace LooCast.System.Collections.Serializable
{
    public interface ISerializableDictionary : IDictionary
    {
        
    }

    public interface ISerializableDictionary<TKey, TValue> : ISerializableDictionary, IDictionary<TKey, TValue>
    {
    }
}
