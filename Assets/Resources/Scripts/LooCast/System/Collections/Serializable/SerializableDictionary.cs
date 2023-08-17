using System;
using System.Collections.Generic;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    [SerializableGenericObject]
    public class SerializableDictionary<TKey, TValue> : Dictionary<TKey, TValue>
    {
        #region Enums
        private enum TypeCombination
        {
            PrimitivePrimitive,
            PrimitiveObject,
            ObjectPrimitive,
            ObjectObject
        }
        #endregion
        
        #region Fields
        private PrimitiveTypeInfo keyPrimitiveTypeInfo;
        private PrimitiveTypeInfo valuePrimitiveTypeInfo;
        private ObjectTypeInfo keyObjectTypeInfo;
        private ObjectTypeInfo valueObjectTypeInfo;
        private TypeCombination typeCombination;
        #endregion

        #region Constructors
        public SerializableDictionary() : base()
        {
            Type keyType = typeof(TKey);
            Type valueType = typeof(TValue);
            bool isKeyTypePrimitive;
            bool isValueTypePrimitive;

            if (SerializationManager.TryGetPrimitiveTypeInfo(keyType, out keyPrimitiveTypeInfo))
            {
                isKeyTypePrimitive = true;
            }
            else if(SerializationManager.TryGetObjectTypeInfo(keyType, out keyObjectTypeInfo))
            {
                isKeyTypePrimitive = false;
            }
            else
            {
                throw new Exception($"Key type '{keyType.FullName}' is neither a primitive type nor an object type!");
            }

            if (SerializationManager.TryGetPrimitiveTypeInfo(valueType, out valuePrimitiveTypeInfo))
            {
                isValueTypePrimitive = true;
            }
            else if (SerializationManager.TryGetObjectTypeInfo(valueType, out valueObjectTypeInfo))
            {
                isValueTypePrimitive = false;
            }
            else
            {
                throw new Exception($"Value type '{valueType.FullName}' is neither a primitive type nor an object type!");
            }

            if (isKeyTypePrimitive && isValueTypePrimitive)
            {
                typeCombination = TypeCombination.PrimitivePrimitive;
            }
            else if (isKeyTypePrimitive && !isValueTypePrimitive)
            {
                typeCombination = TypeCombination.PrimitiveObject;
            }
            else if (!isKeyTypePrimitive && isValueTypePrimitive)
            {
                typeCombination = TypeCombination.ObjectPrimitive;
            }
            else
            {
                typeCombination = TypeCombination.ObjectObject;
            }
        }

        public SerializableDictionary(IEnumerable<KeyValuePair<TKey, TValue>> collection) : base(collection)
        {
            Type keyType = typeof(TKey);
            Type valueType = typeof(TValue);
            bool isKeyTypePrimitive;
            bool isValueTypePrimitive;

            if (SerializationManager.TryGetPrimitiveTypeInfo(keyType, out keyPrimitiveTypeInfo))
            {
                isKeyTypePrimitive = true;
            }
            else if (SerializationManager.TryGetObjectTypeInfo(keyType, out keyObjectTypeInfo))
            {
                isKeyTypePrimitive = false;
            }
            else
            {
                throw new Exception($"Key type '{keyType.FullName}' is neither a primitive type nor an object type!");
            }

            if (SerializationManager.TryGetPrimitiveTypeInfo(valueType, out valuePrimitiveTypeInfo))
            {
                isValueTypePrimitive = true;
            }
            else if (SerializationManager.TryGetObjectTypeInfo(valueType, out valueObjectTypeInfo))
            {
                isValueTypePrimitive = false;
            }
            else
            {
                throw new Exception($"Value type '{valueType.FullName}' is neither a primitive type nor an object type!");
            }

            if (isKeyTypePrimitive && isValueTypePrimitive)
            {
                typeCombination = TypeCombination.PrimitivePrimitive;
            }
            else if (isKeyTypePrimitive && !isValueTypePrimitive)
            {
                typeCombination = TypeCombination.PrimitiveObject;
            }
            else if (!isKeyTypePrimitive && isValueTypePrimitive)
            {
                typeCombination = TypeCombination.ObjectPrimitive;
            }
            else
            {
                typeCombination = TypeCombination.ObjectObject;
            }
        }
        #endregion

        #region Static Methods
        public static void Serialize(string serializableDictionaryName, object serializableDictionary, out XElement serializedDictionary)
        {
            serializedDictionary = new XElement(serializableDictionaryName);
            SerializableDictionary<TKey, TValue> dictionary = (SerializableDictionary<TKey, TValue>)serializableDictionary;
            KeyValuePair<TKey, TValue>[] keyValuePairs = dictionary.ToArray();

            for (int i = 0; i < keyValuePairs.Length; i++)
            {
                XElement serializedKeyValuePair = new XElement($"KeyValuePair[{i}]");
                KeyValuePair<TKey, TValue> keyValuePair = keyValuePairs[i];
                XObject serializedKey = null;
                XObject serializedValue = null;

                switch (dictionary.typeCombination)
                {
                    case TypeCombination.PrimitivePrimitive:
                    {
                        dictionary.keyPrimitiveTypeInfo.SerializeDelegate.Invoke("Key", keyValuePair.Key, out XAttribute _serializedKey);
                        dictionary.valuePrimitiveTypeInfo.SerializeDelegate.Invoke("Value", keyValuePair.Value, out XAttribute _serializedValue);
                    
                        serializedKey = _serializedKey;
                        serializedValue = _serializedValue;
                        break; 
                    }
                    case TypeCombination.PrimitiveObject:
                    {
                        dictionary.keyPrimitiveTypeInfo.SerializeDelegate.Invoke("Key", keyValuePair.Key, out XAttribute _serializedKey);
                        dictionary.valueObjectTypeInfo.SerializeDelegate.Invoke("Value", keyValuePair.Value, out XElement _serializedValue);
                    
                        serializedKey = _serializedKey;
                        serializedValue = _serializedValue;
                        break; 
                    }
                    case TypeCombination.ObjectPrimitive:
                    {
                        dictionary.keyObjectTypeInfo.SerializeDelegate.Invoke("Key", keyValuePair.Key, out XElement _serializedKey);
                        dictionary.valuePrimitiveTypeInfo.SerializeDelegate.Invoke("Value", keyValuePair.Value, out XAttribute _serializedValue);
                    
                        serializedKey = _serializedKey;
                        serializedValue = _serializedValue;
                        break; 
                    }
                    case TypeCombination.ObjectObject:
                    {
                        dictionary.keyObjectTypeInfo.SerializeDelegate.Invoke("Key", keyValuePair.Key, out XElement _serializedKey);
                        dictionary.valueObjectTypeInfo.SerializeDelegate.Invoke("Value", keyValuePair.Value, out XElement _serializedValue);
                    
                        serializedKey = _serializedKey;
                        serializedValue = _serializedValue;
                        break; 
                    }
                }

                serializedKeyValuePair.Add(serializedKey);
                serializedKeyValuePair.Add(serializedValue);
                serializedDictionary.Add(serializedKeyValuePair);
            }
        }

        public static void Deserialize(XElement serializedDictionary, out object serializableDictionary)
        {
            XElement[] serializedDictionaryChildElements = serializedDictionary.Elements().ToArray();
            SerializableDictionary<TKey, TValue> dictionary = new SerializableDictionary<TKey, TValue>();

            for (int i = 0; i < serializedDictionaryChildElements.Length; i++)
            {
                XElement serializedKeyValuePair = serializedDictionaryChildElements[i];
                object key = null;
                object value = null;

                switch (dictionary.typeCombination)
                {
                    case TypeCombination.PrimitivePrimitive:
                    {
                        XAttribute serializedKey = serializedKeyValuePair.Attribute("Key");
                        XAttribute serializedValue = serializedKeyValuePair.Attribute("Value");

                        dictionary.keyPrimitiveTypeInfo.DeserializeDelegate.Invoke(serializedKey, out key);
                        dictionary.valuePrimitiveTypeInfo.DeserializeDelegate.Invoke(serializedValue, out value);
                        break; 
                    }
                    case TypeCombination.PrimitiveObject:
                    {
                        XAttribute serializedKey = serializedKeyValuePair.Attribute("Key");
                        XElement serializedValue = serializedKeyValuePair.Element("Value");

                        dictionary.keyPrimitiveTypeInfo.DeserializeDelegate.Invoke(serializedKey, out key);
                        dictionary.valueObjectTypeInfo.DeserializeDelegate.Invoke(serializedValue, out value);
                        break; 
                    }
                    case TypeCombination.ObjectPrimitive:
                    {
                        XElement serializedKey = serializedKeyValuePair.Element("Key");
                        XAttribute serializedValue = serializedKeyValuePair.Attribute("Value");

                        dictionary.keyObjectTypeInfo.DeserializeDelegate.Invoke(serializedKey, out key);
                        dictionary.valuePrimitiveTypeInfo.DeserializeDelegate.Invoke(serializedValue, out value);
                        break; 
                    }
                    case TypeCombination.ObjectObject:
                    {
                        XElement serializedKey = serializedKeyValuePair.Element("Key");
                        XElement serializedValue = serializedKeyValuePair.Element("Value");

                        dictionary.keyObjectTypeInfo.DeserializeDelegate.Invoke(serializedKey, out key);
                        dictionary.valueObjectTypeInfo.DeserializeDelegate.Invoke(serializedValue, out value);
                        break; 
                    }
                }

                dictionary.Add((TKey)key, (TValue)value);
            }

            serializableDictionary = dictionary;
        }
        #endregion
    }
}
