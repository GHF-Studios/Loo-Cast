using System;
using System.Collections.Generic;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    [SerializableGenericObject]
    public class SerializableList<T> : List<T>
    {
        #region Fields
        private ObjectTypeInfo objectTypeInfo;
        #endregion

        #region Constructors
        public SerializableList() : base()
        {
            Type type = typeof(T);
            
            if (!SerializationManager.Instance.TryGetObjectTypeInfo(type, out objectTypeInfo))
            {
                throw new Exception($"Type '{type}' is not an object type!");
            }
        }

        public SerializableList(IEnumerable<T> collection) : base(collection)
        {
            Type type = typeof(T);

            if (!SerializationManager.Instance.TryGetObjectTypeInfo(type, out objectTypeInfo))
            {
                throw new Exception($"Type '{type}' is not an object type!");
            }
        }
        #endregion

        #region Static Methods
        public static void Serialize(string serializableListName, object serializableList, out XElement serializedList)
        {
            serializedList = new XElement(serializableListName);
            SerializableList<T> list = (SerializableList<T>)serializableList;

            for (int i = 0; i < list.Count; i++)
            {
                list.objectTypeInfo.SerializeDelegate($"Item[{i}]", list[i], out XElement serializedItem);
                serializedList.Add(serializedItem);
            }
        }
        
        public static void Deserialize(XElement serializedList, out object serializableList)
        {
            XElement[] serializedListChildElements = serializedList.Elements().ToArray();
            SerializableList<T> list = new SerializableList<T>();

            for (int i = 0; i < serializedListChildElements.Length; i++)
            {
                list.objectTypeInfo.DeserializeDelegate.Invoke(serializedListChildElements[i], out object deserializedItem);
                list.Add((T)deserializedItem);
            }

            serializableList = list;
        }
        #endregion
    }
}
