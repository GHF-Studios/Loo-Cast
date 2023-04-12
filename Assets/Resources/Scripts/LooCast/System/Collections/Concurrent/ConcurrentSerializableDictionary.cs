
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System.Collections.Concurrent
{
    using global::LooCast.System.Collections.Generic;

    public class ConcurrentSerializableDictionary<KeyType, ValueType> : IDictionary<KeyType, ValueType>
    {
        public ICollection<KeyType> Keys
        {
            get
            {
                lock (lockObject)
                {
                    return ((IDictionary<KeyType, ValueType>)serializableDictionary).Keys;
                }
            }
        }
        public ICollection<ValueType> Values
        {
            get
            {
                lock(lockObject)
                {
                    return ((IDictionary<KeyType, ValueType>)serializableDictionary).Values;
                }
            }
        }
        public int Count
        {
            get
            {
                lock (lockObject)
                {
                    return ((ICollection<KeyValuePair<KeyType, ValueType>>)serializableDictionary).Count;
                }
            }
        }
        public bool IsReadOnly
        {
            get
            {
                lock (lockObject)
                {
                    return ((ICollection<KeyValuePair<KeyType, ValueType>>)serializableDictionary).IsReadOnly;
                }
            }
        }

        [SerializeField] private SerializableDictionary<KeyType, ValueType> serializableDictionary = new SerializableDictionary<KeyType, ValueType>();
        private readonly object lockObject = new object();

        public ConcurrentSerializableDictionary()
        {
            serializableDictionary = new SerializableDictionary<KeyType, ValueType>();
        }

        public ValueType this[KeyType key]
        {
            get
            {
                lock (serializableDictionary)
                {
                    return serializableDictionary[key];
                }
            }
            set
            {
                lock (serializableDictionary)
                {
                    serializableDictionary[key] = value;
                }
            }
        }

        public void Add(KeyType key, ValueType value)
        {
            lock (lockObject)
            {
                ((IDictionary<KeyType, ValueType>)serializableDictionary).Add(key, value);
            }
        }

        public bool ContainsKey(KeyType key)
        {
            lock (lockObject)
            {
                return ((IDictionary<KeyType, ValueType>)serializableDictionary).ContainsKey(key);
            }
        }

        public bool Remove(KeyType key)
        {
            lock (lockObject)
            {
                return ((IDictionary<KeyType, ValueType>)serializableDictionary).Remove(key);
            }
        }

        public bool TryGetValue(KeyType key, out ValueType value)
        {
            lock (lockObject)
            {
                return ((IDictionary<KeyType, ValueType>)serializableDictionary).TryGetValue(key, out value);
            }
        }

        public void Add(KeyValuePair<KeyType, ValueType> item)
        {
            lock (lockObject)
            {
                ((ICollection<KeyValuePair<KeyType, ValueType>>)serializableDictionary).Add(item);
            }
        }

        public void Clear()
        {
            lock (lockObject)
            {
                ((ICollection<KeyValuePair<KeyType, ValueType>>)serializableDictionary).Clear();
            }
        }

        public bool Contains(KeyValuePair<KeyType, ValueType> item)
        {
            lock (lockObject)
            {
                return ((ICollection<KeyValuePair<KeyType, ValueType>>)serializableDictionary).Contains(item);
            }
        }

        public void CopyTo(KeyValuePair<KeyType, ValueType>[] array, int arrayIndex)
        {
            lock (lockObject)
            {
                ((ICollection<KeyValuePair<KeyType, ValueType>>)serializableDictionary).CopyTo(array, arrayIndex);
            }
        }

        public bool Remove(KeyValuePair<KeyType, ValueType> item)
        {
            lock (lockObject)
            {
                return ((ICollection<KeyValuePair<KeyType, ValueType>>)serializableDictionary).Remove(item);
            }
        }

        public IEnumerator<KeyValuePair<KeyType, ValueType>> GetEnumerator()
        {
            lock (lockObject)
            {
                return ((IEnumerable<KeyValuePair<KeyType, ValueType>>)serializableDictionary).GetEnumerator();
            }
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            lock (lockObject)
            {
                return ((IEnumerable)serializableDictionary).GetEnumerator();
            }
        }
    }
}