using System;
using System.Collections.Generic;
using System.Linq;
using Unity.VisualScripting;
using UnityEngine;
using UnityEngine.UIElements;

namespace LooCast.Util.Collections.Generic
{
    [Serializable]
    // Note: Can only be correctly serialized, if T is Serializable, too
    public struct SerializableDictionary<KeyType, ValueType>
    {
        #region Structs
        [Serializable]
        public struct Entry
        {
            public KeyType Key => key;
            public ValueType Value => value;

            [SerializeField] private KeyType key;
            [SerializeField] private ValueType value;

            public Entry(KeyType key, ValueType value)
            {
                this.key = key;
                this.value = value;
            }
        }
        #endregion

        #region Properties
        public Entry[] EntryArray => entryArray;
        public KeyType[] KeyArray => keyArray;
        public ValueType[] ValueArray => valueArray;
        #endregion

        #region Fields
        [SerializeField] private Entry[] entryArray;
        [SerializeField] private KeyType[] keyArray;
        [SerializeField] private ValueType[] valueArray;
        #endregion

        #region Methods
        public bool ContainsKey(KeyType key)
        {
            foreach (KeyType otherKey in keyArray)
            {
                if (otherKey.Equals(key))
                {
                    return true;
                }
            }
            return false;
        }

        public bool ContainsValue(ValueType value)
        {
            foreach (ValueType otherValue in valueArray)
            {
                if (otherValue.Equals(value))
                {
                    return true;
                }
            }
            return false;
        }

        public bool ContainsEntry(Entry entry)
        {
            foreach (Entry otherEntry in entryArray)
            {
                if (otherEntry.Equals(entry))
                {
                    return true;
                }
            }
            return false;
        }

        public void Add(KeyType key, ValueType value)
        {
            if (ContainsKey(key))
            {
                throw new ArgumentException("Already contains key!");
            }

            if (ContainsValue(value))
            {
                throw new ArgumentException("Already contains value!");
            }

            List<Entry> entryList = entryArray.ToList();
            List<KeyType> keyList = keyArray.ToList();
            List<ValueType> valueList = valueArray.ToList();
            entryList.Add(new Entry(key, value));
            keyList.Add(key);
            valueList.Add(value);
            entryArray = entryList.ToArray();
            keyArray = keyList.ToArray();
            valueArray = valueList.ToArray();
        }

        public void Remove(KeyType key)
        {
            if (!ContainsKey(key))
            {
                return;
            }

            List<Entry> entryList = entryArray.ToList();
            List<KeyType> keyList = keyArray.ToList();
            List<ValueType> valueList = valueArray.ToList();
            Entry entry = GetEntry(key);
            entryList.Remove(entry);
            keyList.Remove(entry.Key);
            valueList.Remove(entry.Value);
            entryArray = entryList.ToArray();
            keyArray = keyList.ToArray();
            valueArray = valueList.ToArray();
        }

        public void Remove(ValueType value)
        {
            if (!ContainsValue(value))
            {
                return;
            }

            List<Entry> entryList = entryArray.ToList();
            List<KeyType> keyList = keyArray.ToList();
            List<ValueType> valueList = valueArray.ToList();
            Entry entry = GetEntry(value);
            entryList.Remove(entry);
            keyList.Remove(entry.Key);
            valueList.Remove(entry.Value);
            entryArray = entryList.ToArray();
            keyArray = keyList.ToArray();
            valueArray = valueList.ToArray();
        }

        public void Remove(Entry entry)
        {
            if (!ContainsEntry(entry))
            {
                return;
            }

            List<Entry> entryList = entryArray.ToList();
            List<KeyType> keyList = keyArray.ToList();
            List<ValueType> valueList = valueArray.ToList();
            entryList.Remove(entry);
            keyList.Remove(entry.Key);
            valueList.Remove(entry.Value);
            entryArray = entryList.ToArray();
            keyArray = keyList.ToArray();
            valueArray = valueList.ToArray();
        }

        public Entry GetEntry(KeyType key)
        {
            int? index = GetIndex(key);
            if (index == null)
            {
                throw new ArgumentException("Key could not be found!");
            }
            return GetEntry((int)index);
        }

        public Entry GetEntry(ValueType value)
        {
            int? index = GetIndex(value);
            if (index == null)
            {
                throw new ArgumentException("Value could not be found!");
            }
            return GetEntry((int)index);
        }

        private Entry GetEntry(int index)
        {
            if (index < 0)
            {
                throw new ArgumentOutOfRangeException("Index cannot be smaller than 0!");
            }
            if (index >= entryArray.Length)
            {
                throw new ArgumentOutOfRangeException("Index cannot be greater than or equal to the array!");
            }

            return entryArray[index];
        }
        
        private int? GetIndex(KeyType key)
        {
            for (int i = 0; i < entryArray.Length; i++)
            {
                if (entryArray[i].Key.Equals(key))
                {
                    return i;
                }
            }
            return null;
        }

        private int? GetIndex(ValueType value)
        {
            for (int i = 0; i < entryArray.Length; i++)
            {
                if (entryArray[i].Value.Equals(value))
                {
                    return i;
                }
            }
            return null;
        }
        #endregion
    }
}