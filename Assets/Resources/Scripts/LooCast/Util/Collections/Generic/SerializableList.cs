using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.Util.Collections.Generic
{
    [Serializable]
    public struct SerializableList<T>
    {
        public List<T> Values
        {
            get
            {
                return values.ToList();
            }
        }

        [SerializeField] private T[] values;

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
            return values.Contains(value);
        }
    }
}
