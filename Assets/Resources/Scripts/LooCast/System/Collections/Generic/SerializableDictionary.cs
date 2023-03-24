using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System.Collections.Generic
{
    [Serializable]
    public class SerializableDictionary<KeyType, ValueType> : IDictionary<KeyType, ValueType>, ISerializationCallbackReceiver
    {
        [SerializeField] private SerializableList<KeyType> keys = new SerializableList<KeyType>();
        [SerializeField] private SerializableList<ValueType> values = new SerializableList<ValueType>();
        
        private Dictionary<KeyType, ValueType> hashTable = new Dictionary<KeyType, ValueType>();

        public ICollection<KeyType> Keys => ((IDictionary<KeyType, ValueType>)hashTable).Keys;
        public ICollection<ValueType> Values => ((IDictionary<KeyType, ValueType>)hashTable).Values;
        public int Count => ((ICollection<KeyValuePair<KeyType, ValueType>>)hashTable).Count;
        public bool IsReadOnly => ((ICollection<KeyValuePair<KeyType, ValueType>>)hashTable).IsReadOnly;

        public ValueType this[KeyType key]
        {
            get => hashTable[key];
            set => hashTable[key] = value;
        }

        public void OnBeforeSerialize()
        {
            keys.Clear();
            values.Clear();

            foreach (var kvp in hashTable)
            {
                keys.Add(kvp.Key);
                values.Add(kvp.Value);
            }
        }

        public void OnAfterDeserialize()
        {
            hashTable.Clear();

            for (int i = 0; i < keys.Count; i++)
            {
                hashTable.Add(keys[i], values[i]);
            }
        }

        public void Add(KeyType key, ValueType value)
        {
            ((IDictionary<KeyType, ValueType>)hashTable).Add(key, value);
        }

        public bool ContainsKey(KeyType key)
        {
            return ((IDictionary<KeyType, ValueType>)hashTable).ContainsKey(key);
        }

        public bool Remove(KeyType key)
        {
            return ((IDictionary<KeyType, ValueType>)hashTable).Remove(key);
        }

        public bool TryGetValue(KeyType key, out ValueType value)
        {
            return ((IDictionary<KeyType, ValueType>)hashTable).TryGetValue(key, out value);
        }

        public void Add(KeyValuePair<KeyType, ValueType> item)
        {
            ((ICollection<KeyValuePair<KeyType, ValueType>>)hashTable).Add(item);
        }

        public void Clear()
        {
            ((ICollection<KeyValuePair<KeyType, ValueType>>)hashTable).Clear();
        }

        public bool Contains(KeyValuePair<KeyType, ValueType> item)
        {
            return ((ICollection<KeyValuePair<KeyType, ValueType>>)hashTable).Contains(item);
        }

        public void CopyTo(KeyValuePair<KeyType, ValueType>[] array, int arrayIndex)
        {
            ((ICollection<KeyValuePair<KeyType, ValueType>>)hashTable).CopyTo(array, arrayIndex);
        }

        public bool Remove(KeyValuePair<KeyType, ValueType> item)
        {
            return ((ICollection<KeyValuePair<KeyType, ValueType>>)hashTable).Remove(item);
        }

        public IEnumerator<KeyValuePair<KeyType, ValueType>> GetEnumerator()
        {
            return ((IEnumerable<KeyValuePair<KeyType, ValueType>>)hashTable).GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return ((IEnumerable)hashTable).GetEnumerator();
        }
    }
}