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
        public Entry[] EntryArray
        {
            get
            {
                if (entryArray == null)
                {
                    entryArray = new Entry[0];
                }
                return entryArray;
            }

            set
            {
                entryArray = value;
            }
        }
        public KeyType[] KeyArray
        {
            get
            {
                if (keyArray == null)
                {
                    keyArray = new KeyType[0];
                }
                return keyArray;
            }

            set
            {
                keyArray = value;
            }
        }
        public ValueType[] ValueArray
        {
            get
            {
                if (valueArray == null)
                {
                    valueArray = new ValueType[0];
                }
                return valueArray;
            }

            set
            {
                valueArray = value;
            }
        }
        #endregion

        #region Fields
        [SerializeField] private Entry[] entryArray;
        [SerializeField] private KeyType[] keyArray;
        [SerializeField] private ValueType[] valueArray;
        #endregion

        #region Methods
        public bool ContainsKey(KeyType key)
        {
            foreach (KeyType otherKey in KeyArray)
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
            foreach (ValueType otherValue in ValueArray)
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
            foreach (Entry otherEntry in EntryArray)
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

            List<Entry> entryList = EntryArray.ToList();
            List<KeyType> keyList = KeyArray.ToList();
            List<ValueType> valueList = ValueArray.ToList();
            entryList.Add(new Entry(key, value));
            keyList.Add(key);
            valueList.Add(value);
            EntryArray = entryList.ToArray();
            KeyArray = keyList.ToArray();
            ValueArray = valueList.ToArray();
        }

        public void Remove(KeyType key)
        {
            if (!ContainsKey(key))
            {
                return;
            }

            List<Entry> entryList = EntryArray.ToList();
            List<KeyType> keyList = KeyArray.ToList();
            List<ValueType> valueList = ValueArray.ToList();
            Entry entry = GetEntry(key);
            entryList.Remove(entry);
            keyList.Remove(entry.Key);
            valueList.Remove(entry.Value);
            EntryArray = entryList.ToArray();
            KeyArray = keyList.ToArray();
            ValueArray = valueList.ToArray();
        }

        public void Remove(ValueType value)
        {
            if (!ContainsValue(value))
            {
                return;
            }

            List<Entry> entryList = EntryArray.ToList();
            List<KeyType> keyList = KeyArray.ToList();
            List<ValueType> valueList = ValueArray.ToList();
            Entry entry = GetEntry(value);
            entryList.Remove(entry);
            keyList.Remove(entry.Key);
            valueList.Remove(entry.Value);
            EntryArray = entryList.ToArray();
            KeyArray = keyList.ToArray();
            ValueArray = valueList.ToArray();
        }

        public void Remove(Entry entry)
        {
            if (!ContainsEntry(entry))
            {
                return;
            }

            List<Entry> entryList = EntryArray.ToList();
            List<KeyType> keyList = KeyArray.ToList();
            List<ValueType> valueList = ValueArray.ToList();
            entryList.Remove(entry);
            keyList.Remove(entry.Key);
            valueList.Remove(entry.Value);
            EntryArray = entryList.ToArray();
            KeyArray = keyList.ToArray();
            ValueArray = valueList.ToArray();
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
            if (index >= EntryArray.Length)
            {
                throw new ArgumentOutOfRangeException("Index cannot be greater than or equal to the array!");
            }

            return EntryArray[index];
        }
        
        private int? GetIndex(KeyType key)
        {
            for (int i = 0; i < EntryArray.Length; i++)
            {
                if (EntryArray[i].Key.Equals(key))
                {
                    return i;
                }
            }
            return null;
        }

        private int? GetIndex(ValueType value)
        {
            for (int i = 0; i < EntryArray.Length; i++)
            {
                if (EntryArray[i].Value.Equals(value))
                {
                    return i;
                }
            }
            return null;
        }
        #endregion
    }
}