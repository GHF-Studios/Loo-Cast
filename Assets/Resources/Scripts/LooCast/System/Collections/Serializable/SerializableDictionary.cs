using System;
using System.Collections.Generic;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    [SerializableObject(true, true)]
    public class SerializableDictionary<TKey, TValue> : Dictionary<TKey, TValue>, ISerializableDictionary<TKey, TValue>
    {
        #region Fields
        private Serializability keyTypeSerializability;
        private Serializability valueTypeSerializability;
        
        private Type keyType;
        private Type valueType;

        private SerializationManager.SerializePrimitiveDelegate serializeKeyPrimitiveDelegate;
        private SerializationManager.DeserializePrimitiveDelegate deserializeKeyPrimitiveDelegate;

        private SerializationManager.SerializePrimitiveDelegate serializeValuePrimitiveDelegate;
        private SerializationManager.DeserializePrimitiveDelegate deserializeValuePrimitiveDelegate;

        private SerializationManager.SerializeObjectDelegate serializeKeyObjectDelegate;
        private SerializationManager.DeserializeObjectDelegate deserializeKeyObjectDelegate;

        private SerializationManager.SerializeObjectDelegate serializeValueObjectDelegate;
        private SerializationManager.DeserializeObjectDelegate deserializeValueObjectDelegate;
        #endregion

        #region Constructors
        public SerializableDictionary() : base()
        {
            SerializationManager serializationManager = SerializationManager.Instance;
            
            keyType = typeof(TKey);
            valueType = typeof(TValue);
            
            keyTypeSerializability = serializationManager.GetSerializability(keyType);
            valueTypeSerializability = serializationManager.GetSerializability(valueType);
            
            switch (keyTypeSerializability)
            {
                case Serializability.None:
                    throw new ArgumentException($"The key type '{keyType.Name}' is not serializable!");
                case Serializability.Primitive:
                    serializeKeyPrimitiveDelegate = serializationManager.GetPrimitiveSerializationDelegate(keyType);
                    deserializeKeyPrimitiveDelegate = serializationManager.GetPrimitiveDeserializationDelegate(keyType);
                    serializeKeyObjectDelegate = null;
                    deserializeKeyObjectDelegate = null;
                    break;
                case Serializability.Object:
                    serializeKeyPrimitiveDelegate = null;
                    deserializeKeyPrimitiveDelegate = null;
                    serializeKeyObjectDelegate = serializationManager.GetObjectSerializationDelegate(keyType);
                    deserializeKeyObjectDelegate = serializationManager.GetObjectDeserializationDelegate(keyType);
                    break;
                case Serializability.File:
                    throw new InvalidOperationException("A serializable dictionary cannot contain files as keys, only attributes or objects as keys!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable dictionary cannot contain folders as keys, only attributes or objects as keys!");
            }
            switch (valueTypeSerializability)
            {
                case Serializability.None:
                    throw new ArgumentException($"The value type '{valueType.Name}' is not serializable!");
                case Serializability.Primitive:
                    serializeValuePrimitiveDelegate = serializationManager.GetPrimitiveSerializationDelegate(valueType);
                    deserializeValuePrimitiveDelegate = serializationManager.GetPrimitiveDeserializationDelegate(valueType);
                    serializeValueObjectDelegate = null;
                    deserializeValueObjectDelegate = null;
                    break;
                case Serializability.Object:
                    serializeValuePrimitiveDelegate = null;
                    deserializeValuePrimitiveDelegate = null;
                    serializeValueObjectDelegate = serializationManager.GetObjectSerializationDelegate(valueType);
                    deserializeValueObjectDelegate = serializationManager.GetObjectDeserializationDelegate(valueType);
                    break;
                case Serializability.File:
                    throw new InvalidOperationException("A serializable dictionary cannot contain files as values, only attributes or objects as values!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable dictionary cannot contain folders as values, only attributes or objects as values!");
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
                XObject serializedKey;
                XObject serializedValue;

                if (dictionary.keyTypeSerializability == Serializability.Primitive && dictionary.valueTypeSerializability == Serializability.Primitive)
                {
                    dictionary.serializeKeyPrimitiveDelegate.Invoke("Key", keyValuePair.Key, out XAttribute _serializedKey);
                    dictionary.serializeValuePrimitiveDelegate.Invoke("Value", keyValuePair.Value, out XAttribute _serializedValue);

                    serializedKey = _serializedKey;
                    serializedValue = _serializedValue;
                }
                else if (dictionary.keyTypeSerializability == Serializability.Primitive && dictionary.valueTypeSerializability == Serializability.Object)
                {
                    dictionary.serializeKeyPrimitiveDelegate.Invoke("Key", keyValuePair.Key, out XAttribute _serializedKey);
                    dictionary.serializeValueObjectDelegate.Invoke("Value", keyValuePair.Value, out XElement _serializedValue);

                    serializedKey = _serializedKey;
                    serializedValue = _serializedValue;
                }
                else if (dictionary.keyTypeSerializability == Serializability.Object && dictionary.valueTypeSerializability == Serializability.Primitive)
                {
                    dictionary.serializeKeyObjectDelegate.Invoke("Key", keyValuePair.Key, out XElement _serializedKey);
                    dictionary.serializeValuePrimitiveDelegate.Invoke("Value", keyValuePair.Value, out XAttribute _serializedValue);

                    serializedKey = _serializedKey;
                    serializedValue = _serializedValue;
                }
                else
                {
                    dictionary.serializeKeyObjectDelegate.Invoke("Key", keyValuePair.Key, out XElement _serializedKey);
                    dictionary.serializeValueObjectDelegate.Invoke("Value", keyValuePair.Value, out XElement _serializedValue);

                    serializedKey = _serializedKey;
                    serializedValue = _serializedValue;
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
                object key;
                object value;
                
                if (dictionary.keyTypeSerializability == Serializability.Primitive && dictionary.valueTypeSerializability == Serializability.Primitive)
                {
                    XAttribute serializedKey = serializedKeyValuePair.Attribute("Key");
                    XAttribute serializedValue = serializedKeyValuePair.Attribute("Value");
                    
                    dictionary.deserializeKeyPrimitiveDelegate.Invoke(serializedKey, out key);
                    dictionary.deserializeValuePrimitiveDelegate.Invoke(serializedValue, out value);
                }
                else if (dictionary.keyTypeSerializability == Serializability.Primitive && dictionary.valueTypeSerializability == Serializability.Object)
                {
                    XAttribute serializedKey = serializedKeyValuePair.Attribute("Key");
                    XElement serializedValue = serializedKeyValuePair.Element("Value");

                    dictionary.deserializeKeyPrimitiveDelegate.Invoke(serializedKey, out key);
                    dictionary.deserializeValueObjectDelegate.Invoke(serializedValue, out value);
                }
                else if (dictionary.keyTypeSerializability == Serializability.Object && dictionary.valueTypeSerializability == Serializability.Primitive)
                {
                    XElement serializedKey = serializedKeyValuePair.Element("Key");
                    XAttribute serializedValue = serializedKeyValuePair.Attribute("Value");

                    dictionary.deserializeKeyObjectDelegate.Invoke(serializedKey, out key);
                    dictionary.deserializeValuePrimitiveDelegate.Invoke(serializedValue, out value);
                }
                else
                {
                    XElement serializedKey = serializedKeyValuePair.Element("Key");
                    XElement serializedValue = serializedKeyValuePair.Element("Value");

                    dictionary.deserializeKeyObjectDelegate.Invoke(serializedKey, out key);
                    dictionary.deserializeValueObjectDelegate.Invoke(serializedValue, out value);
                }
                
                dictionary.Add((TKey)key, (TValue)value);
            }

            serializableDictionary = dictionary;
        }
        #endregion
    }
}
