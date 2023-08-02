using System;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    public class SerializableArray<T> : ISerializableObject where T : new()
    {
        #region Properties
        public T[] Array { get; private set; }
        #endregion
        
        #region Fields
        private Serializability typeSerializability;
        private Type type;
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
                case Serializability.PrimitiveAttribute:
                    throw new InvalidOperationException("A serializable array cannot contain attributes, only objects, as the order of the attributes cannot be preserved!");
                case Serializability.Attribute:
                    throw new InvalidOperationException("A serializable array cannot contain attributes, only objects, as the order of the attributes cannot be preserved!");
                case Serializability.File:
                    throw new InvalidOperationException("A serializable array cannot contain files, only objects!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable array cannot contain folders, only objects!");
            }
            Array = new T[length];
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
                case Serializability.PrimitiveAttribute:
                    throw new InvalidOperationException("A serializable array cannot contain attributes, only objects, as the order of the attributes cannot be preserved!");
                case Serializability.Attribute:
                    throw new InvalidOperationException("A serializable array cannot contain attributes, only objects, as the order of the attributes cannot be preserved!");
                case Serializability.File:
                    throw new InvalidOperationException("A serializable array cannot contain files, only objects!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable array cannot contain folders, only objects!");
            }
            Array = array.Clone() as T[];
        }
        #endregion

        #region Methods
        public void Serialize(string arrayName, out XElement serializedArray)
        {
            serializedArray = new XElement(arrayName);
            SerializationManager serializationManager = SerializationManager.Instance;
            switch (typeSerializability)
            {
                case Serializability.PrimitiveObject:
                {
                    IPrimitiveObjectSerializer primitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(type);
                    
                    for (int i = 0; i < Array.Length; i++)
                    {
                        primitiveObjectSerializer.Serialize($"Item[{i}]", Array[i], out XElement serializedItem);
                        serializedArray.Add(serializedItem);
                    }
                    
                    break;
                }
                case Serializability.Object:
                {
                    for (int i = 0; i < Array.Length; i++)
                    {
                        ((ISerializableObject)Array[i]).Serialize($"Item[{i}]", out XElement serializedItem);
                        serializedArray.Add(serializedItem);
                    }
                    
                    break;
                }
            }
        }

        public void Deserialize(XElement serializedArray)
        {
            SerializationManager serializationManager = SerializationManager.Instance;
            switch (typeSerializability)
            {
                case Serializability.PrimitiveObject:
                {
                    IPrimitiveObjectSerializer primitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(type);
                    XElement[] serializedObjectChildElements = serializedArray.Elements().ToArray();
                    
                    if (Array.Length < serializedObjectChildElements.Length)
                    {
                        throw new InvalidOperationException("The array does not have enough capacity for the deserialized data!");
                    }
                    
                    for (int i = 0; i < serializedObjectChildElements.Length; i++)
                    {
                        primitiveObjectSerializer.Deserialize(serializedObjectChildElements[i], out object item);
                        Array[i] = (T)item;
                    }
                    
                    break;
                }
                case Serializability.Object:
                {
                    XElement[] serializedObjectChildElements = serializedArray.Elements().ToArray();
                    
                    if (Array.Length < serializedObjectChildElements.Length)
                    {
                        throw new InvalidOperationException("The array does not have enough capacity for the deserialized data!");
                    }
                    
                    for (int i = 0; i < serializedObjectChildElements.Length; i++)
                    {
                        ISerializableObject item = (ISerializableObject)new T();
                        item.Deserialize(serializedObjectChildElements[i]);
                        Array[i] = (T)item;
                    }

                    break;
                }
            }
        }
        #endregion
    }
}
