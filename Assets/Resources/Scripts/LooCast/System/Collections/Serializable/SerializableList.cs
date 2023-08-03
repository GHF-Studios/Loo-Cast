using System;
using System.Collections.Generic;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    [SerializableObject(true, true)]
    public class SerializableList<T> : List<T>, ISerializableList<T>
    {
        #region Fields
        private Serializability typeSerializability;
        private Type type;

        private SerializationManager.SerializeObjectDelegate serializeObjectDelegate;
        private SerializationManager.DeserializeObjectDelegate deserializeObjectDelegate;
        #endregion

        #region Constructors
        public SerializableList() : base()
        {
            type = typeof(T);
            SerializationManager serializationManager = SerializationManager.Instance;
            typeSerializability = serializationManager.GetSerializability(type);
            switch (typeSerializability)
            {
                case Serializability.None:
                    throw new ArgumentException($"The type '{type.Name}' is not serializable!");
                case Serializability.Primitive:
                    throw new InvalidOperationException("A serializable list cannot contain primitives, only objects!");
                case Serializability.File:
                    throw new InvalidOperationException("A serializable list cannot contain files, only attributes or objects!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable list cannot contain folders, only attributes or objects!");
            }
            serializeObjectDelegate = serializationManager.GetObjectSerializationDelegate(type);
            deserializeObjectDelegate = serializationManager.GetObjectDeserializationDelegate(type);
        }
        #endregion

        #region Static Methods
        public static void Serialize(string serializableListName, object serializableList, out XElement serializedList)
        {
            serializedList = new XElement(serializableListName);
            SerializableList<T> list = (SerializableList<T>)serializableList;

            for (int i = 0; i < list.Count; i++)
            {
                list.serializeObjectDelegate($"Item[{i}]", list[i], out XElement serializedItem);
                serializedList.Add(serializedItem);
            }
        }
        
        public static void Deserialize(XElement serializedList, out object serializableList)
        {
            XElement[] serializedListChildElements = serializedList.Elements().ToArray();
            SerializableList<T> list = new SerializableList<T>();

            for (int i = 0; i < serializedListChildElements.Length; i++)
            {
                list.deserializeObjectDelegate.Invoke(serializedListChildElements[i], out object deserializedItem);
                list.Add((T)deserializedItem);
            }

            serializableList = list;
        }
        #endregion
    }
}
