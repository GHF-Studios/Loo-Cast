using System;
using System.Collections.Generic;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    public class SerializableList<T> : List<T>, ISerializableList<T> where T : new()
    {
        #region Fields
        private Serializability typeSerializability;
        private Type type;
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
                case Serializability.File:
                    throw new InvalidOperationException("A serializable list cannot contain files, only attributes or objects!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable list cannot contain folders, only attributes or objects!");
            }
        }
        #endregion

        #region Methods
        public void Serialize(string listName, out XElement serializedList)
        {
            serializedList = new XElement(listName);
            SerializationManager serializationManager = SerializationManager.Instance;
            switch (typeSerializability)
            {
                case Serializability.PrimitiveAttribute:
                {
                    IPrimitiveAttributeSerializer primitiveAttributeSerializer = serializationManager.GetPrimitiveAttributeSerializer(type);
                    
                    for (int i = 0; i < Count; i++)
                    {
                        primitiveAttributeSerializer.Serialize($"Item[{i}]", this[i], out XAttribute serializedItem);
                        serializedList.Add(serializedItem);
                    }
                    
                    break;
                }
                case Serializability.PrimitiveObject:
                {
                    IPrimitiveObjectSerializer primitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(type);
                    
                    for (int i = 0; i < Count; i++)
                    {
                        primitiveObjectSerializer.Serialize($"Item[{i}]", this[i], out XElement serializedItem);
                        serializedList.Add(serializedItem);
                    }

                    break;
                }
                case Serializability.Attribute:
                {
                    for (int i = 0; i < Count; i++)
                    {
                        ((ISerializableAttribute)this[i]).Serialize($"Item[{i}]", out XAttribute serializedItem);
                        serializedList.Add(serializedItem);
                    }
                    
                    break;
                }
                case Serializability.Object:
                {
                    for (int i = 0; i < Count; i++)
                    {
                        ((ISerializableObject)this[i]).Serialize($"Item[{i}]", out XElement serializedItem);
                        serializedList.Add(serializedItem);
                    }
                    
                    break;
                }
            }
        }
        
        public void Deserialize(XElement serializedList)
        {
            Clear();
            switch (typeSerializability)
            {
                case Serializability.PrimitiveAttribute:
                {
                    SerializationManager serializationManager = SerializationManager.Instance;
                    IPrimitiveAttributeSerializer primitiveAttributeSerializer = serializationManager.GetPrimitiveAttributeSerializer(type);
                    XAttribute[] serializedObjectAttributes = serializedList.Attributes().ToArray();
                    
                    for (int i = 0; i < serializedObjectAttributes.Length; i++)
                    {
                        primitiveAttributeSerializer.Deserialize(serializedObjectAttributes[i], out object item);
                        Add((T)item);
                    }
                    
                    break;
                }
                case Serializability.PrimitiveObject:
                {
                    SerializationManager serializationManager = SerializationManager.Instance;
                    IPrimitiveObjectSerializer primitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(type);
                    XElement[] serializedObjectChildElements = serializedList.Elements().ToArray();
                    
                    for (int i = 0; i < serializedObjectChildElements.Length; i++)
                    {
                        primitiveObjectSerializer.Deserialize(serializedObjectChildElements[i], out object item);
                        Add((T)item);
                    }
                    
                    break;
                }
                case Serializability.Attribute:
                {
                    XAttribute[] serializedObjectAttributes = serializedList.Attributes().ToArray();
                    
                    for (int i = 0; i < serializedObjectAttributes.Length; i++)
                    {
                        ISerializableAttribute item = (ISerializableAttribute)new T();
                        item.Deserialize(serializedObjectAttributes[i]);
                        Add((T)item);
                    }
                    
                    break;
                }
                case Serializability.Object:
                {
                    XElement[] serializedObjectChildElements = serializedList.Elements().ToArray();
                    
                    for (int i = 0; i < serializedObjectChildElements.Length; i++)
                    {
                        ISerializableObject item = (ISerializableObject)new T();
                        item.Deserialize(serializedObjectChildElements[i]);
                        Add((T)item);
                    }
                    
                    break;
                }
            }
        }
        #endregion
    }
}
