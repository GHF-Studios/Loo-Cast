using System;
using System.Linq;
using System.Xml.Linq;
using System.Collections;
using System.Collections.Generic;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    [SerializableGenericObject]
    public class SerializableArray<T> : IEnumerable, IEnumerable<T>
    {
        #region Properties
        public T[] Array { get; private set; }
        public int Length => Array.Length;
        public object this[int index] { get => Array[index]; set => Array[index] = (T)value; }
        #endregion

        #region Fields
        private ObjectTypeInfo objectTypeInfo;
        #endregion

        #region Constructors
        public SerializableArray(int length) : base()
        {
            Type type = typeof(T);

            if (!SerializationManager.Instance.TryGetObjectTypeInfo(type, out objectTypeInfo))
            {
                throw new Exception($"Type '{type}' is not an object type!");
            }
            
            Array = new T[length];
        }
        
        public SerializableArray(T[] array) : base()
        {
            Type type = typeof(T);

            if (!SerializationManager.Instance.TryGetObjectTypeInfo(type, out objectTypeInfo))
            {
                throw new Exception($"Type '{type}' is not an object type!");
            }

            Array = array.Clone() as T[];
        }

        private SerializableArray()
        {
            Type type = typeof(T);

            if (!SerializationManager.Instance.TryGetObjectTypeInfo(type, out objectTypeInfo))
            {
                throw new Exception($"Type '{type}' is not an object type!");
            }

            Array = global::System.Array.Empty<T>();
        }
        #endregion

        #region Static Methods
        public static void Serialize(string serializableArrayName, object serializableArray, out XElement serializedArray)
        {
            serializedArray = new XElement(serializableArrayName);
            SerializableArray<T> array = (SerializableArray<T>)serializableArray;

            for (int i = 0; i < array.Length; i++)
            {
                array.objectTypeInfo.SerializeDelegate.Invoke($"Item[{i}]", array[i], out XElement serializedItem);
                serializedArray.Add(serializedItem);
            }
        }

        public static void Deserialize(XElement serializedArray, out object serializableArray)
        {
            XElement[] serializedArrayChildElements = serializedArray.Elements().ToArray();
            SerializableArray<T> array = new SerializableArray<T>(serializedArrayChildElements.Length);

            for (int i = 0; i < serializedArrayChildElements.Length; i++)
            {
                array.objectTypeInfo.DeserializeDelegate.Invoke(serializedArrayChildElements[i], out object deserializedItem);
                array[i] = (T)deserializedItem;
            }

            serializableArray = array;
        }

        public static SerializableArray<T> Empty()
        {
            return new SerializableArray<T>();
        }
        #endregion

        #region Methods
        IEnumerator<T> IEnumerable<T>.GetEnumerator()
        {
            return ((IEnumerable<T>)Array).GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return Array.GetEnumerator();
        }
        #endregion
    }
}
