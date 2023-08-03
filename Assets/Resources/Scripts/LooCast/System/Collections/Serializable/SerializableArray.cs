using System;
using System.Linq;
using System.Xml.Linq;
using System.Collections;
using System.Collections.Generic;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    [SerializableObject(true, true)]
    public class SerializableArray<T> : ISerializableArray<T>
    {
        #region Properties
        public T[] Array { get; private set; }
        public int Length => Array.Length;
        public object this[int index] { get => Array[index]; set => Array[index] = (T)value; }
        #endregion

        #region Fields
        private Serializability typeSerializability;
        private Type type;

        private SerializationManager.SerializeObjectDelegate serializeObjectDelegate;
        private SerializationManager.DeserializeObjectDelegate deserializeObjectDelegate;
        #endregion

        #region Constructors
        public SerializableArray(int length) : base()
        {
            type = typeof(T);
            SerializationManager serializationManager = SerializationManager.Instance;
            typeSerializability = serializationManager.GetSerializability(type);
            switch (typeSerializability)
            {
                case Serializability.None:
                    throw new ArgumentException($"The type '{type.Name}' is not serializable!");
                case Serializability.Primitive:
                    throw new InvalidOperationException("A serializable array cannot contain primitives, only objects!");
                case Serializability.File:
                    throw new InvalidOperationException("A serializable array cannot contain files, only objects!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable array cannot contain folders, only objects!");
            }
            Array = new T[length];
            serializeObjectDelegate = serializationManager.GetObjectSerializationDelegate(type);
            deserializeObjectDelegate = serializationManager.GetObjectDeserializationDelegate(type);
        }
        
        public SerializableArray(T[] array) : base()
        {
            type = typeof(T);
            SerializationManager serializationManager = SerializationManager.Instance;
            typeSerializability = serializationManager.GetSerializability(type);
            switch (typeSerializability)
            {
                case Serializability.None:
                    throw new ArgumentException($"The type '{type.Name}' is not serializable!");
                case Serializability.Primitive:
                    throw new InvalidOperationException("A serializable array cannot contain primitives, only objects!");
                case Serializability.File:
                    throw new InvalidOperationException("A serializable array cannot contain files, only objects!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable array cannot contain folders, only objects!");
            }
            Array = array.Clone() as T[];
            serializeObjectDelegate = serializationManager.GetObjectSerializationDelegate(type);
            deserializeObjectDelegate = serializationManager.GetObjectDeserializationDelegate(type);
        }

        private SerializableArray()
        {
            type = typeof(T);
            SerializationManager serializationManager = SerializationManager.Instance;
            typeSerializability = serializationManager.GetSerializability(type);
            switch (typeSerializability)
            {
                case Serializability.None:
                    throw new ArgumentException($"The type '{type.Name}' is not serializable!");
                case Serializability.Primitive:
                    throw new InvalidOperationException("A serializable array cannot contain primitives, only objects!");
                case Serializability.File:
                    throw new InvalidOperationException("A serializable array cannot contain files, only objects!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable array cannot contain folders, only objects!");
            }
            Array = global::System.Array.Empty<T>();
            serializeObjectDelegate = serializationManager.GetObjectSerializationDelegate(type);
            deserializeObjectDelegate = serializationManager.GetObjectDeserializationDelegate(type);
        }
        #endregion

        #region Static Methods
        public static void Serialize(string serializableArrayName, object serializableArray, out XElement serializedArray)
        {
            serializedArray = new XElement(serializableArrayName);
            SerializableArray<T> array = (SerializableArray<T>)serializableArray;

            for (int i = 0; i < array.Length; i++)
            {
                array.serializeObjectDelegate.Invoke($"Item[{i}]", array[i], out XElement serializedItem);
                serializedArray.Add(serializedItem);
            }
        }

        public static void Deserialize(XElement serializedArray, out object serializableArray)
        {
            XElement[] serializedArrayChildElements = serializedArray.Elements().ToArray();
            SerializableArray<T> array = new SerializableArray<T>(serializedArrayChildElements.Length);

            for (int i = 0; i < serializedArrayChildElements.Length; i++)
            {
                array.deserializeObjectDelegate.Invoke(serializedArrayChildElements[i], out object deserializedItem);
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
