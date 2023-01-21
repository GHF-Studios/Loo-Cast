using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.Util.Collections.Generic
{
    [Serializable]
    public class SerializableList<T>
    {
        public List<T> Values
        {
            get
            {
                if (values == null)
                {
                    return new List<T>();
                }
                else
                {
                    return values.ToList();
                }
            }
        }
        public int Count
        {
            get
            {
                if (values == null)
                {
                    return 0;
                }
                else
                {
                    return values.Length;
                }
            }
        }

        [SerializeField] private T[] values;

        public SerializableList()
        {
            values = null;
        }

        public SerializableList(List<T> values)
        {
            this.values = values.ToArray();
        }

        public void Add(T value)
        {
            List<T> valueList = Values;
            valueList.Add(value);
            values = valueList.ToArray();
        }

        public void Remove(T value)
        {
            List<T> valueList = Values;
            valueList.Remove(value);
            values = valueList.ToArray();
        }

        public bool Contains(T value)
        {
            return Values.Contains(value);
        }
    }
}
