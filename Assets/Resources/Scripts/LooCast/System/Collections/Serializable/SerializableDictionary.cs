using System;
using System.Collections.Generic;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Collections.Serializable
{
    using LooCast.System.Serialization;

    public class SerializableDictionary<TKey, TValue> : Dictionary<TKey, TValue>, ISerializableDictionary<TKey, TValue> where TKey : new() where TValue : new()
    {
        #region Fields
        private Serializability keyTypeSerializability;
        private Serializability valueTypeSerializability;
        private Type keyType;
        private Type valueType;
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
                case Serializability.PrimitiveAttribute:
                    throw new InvalidOperationException("A serializable dictionary cannot contain attributes as keys, only objects as keys, as the order of the attribute keys cannot be preserved!");
                case Serializability.Attribute:
                    throw new InvalidOperationException("A serializable dictionary cannot contain attributes as keys, only objects as keys, as the order of the attribute keys cannot be preserved!");
                case Serializability.File:
                    throw new InvalidOperationException("A serializable dictionary cannot contain files as keys, only objects as keys!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable dictionary cannot contain folders as keys, only objects as keys!");
            }
            switch (valueTypeSerializability)
            {
                case Serializability.None:
                    throw new ArgumentException($"The value type '{valueType.Name}' is not serializable!");
                case Serializability.PrimitiveAttribute:
                    throw new InvalidOperationException("A serializable dictionary cannot contain attributes as keys, only objects as keys, as the order of the attribute keys cannot be preserved!");
                case Serializability.Attribute:
                    throw new InvalidOperationException("A serializable dictionary cannot contain attributes as keys, only objects as keys, as the order of the attribute keys cannot be preserved!");
                case Serializability.File:
                    throw new InvalidOperationException("A serializable dictionary cannot contain files as keys, only objects as keys!");
                case Serializability.Folder:
                    throw new InvalidOperationException("A serializable dictionary cannot contain folders as keys, only objects as keys!");
            }
        }
        #endregion

        #region Methods
        public void Serialize(string dictionaryName, out XElement serializedDictionary)
        {
            serializedDictionary = new XElement(dictionaryName);
            if (keyTypeSerializability == Serializability.PrimitiveObject && valueTypeSerializability == Serializability.PrimitiveObject)
            {
                SerializationManager serializationManager = SerializationManager.Instance;
                IPrimitiveObjectSerializer keyPrimitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(keyType);
                IPrimitiveObjectSerializer valuePrimitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(valueType);
                KeyValuePair<TKey, TValue>[] keyValuePairs = this.ToArray();
                
                for (int i = 0; i < Count; i++)
                {
                    XElement serializedKeyValuePair = new XElement($"KeyValuePair[{i}]");
                    KeyValuePair<TKey, TValue> keyValuePair = keyValuePairs[i];
                    
                    keyPrimitiveObjectSerializer.Serialize("Key", keyValuePair.Key, out XElement serializedKey);
                    valuePrimitiveObjectSerializer.Serialize("Value", keyValuePair.Value, out XElement serializedValue);
                    
                    serializedKeyValuePair.Add(serializedKey);
                    serializedKeyValuePair.Add(serializedValue);
                    serializedDictionary.Add(serializedKeyValuePair);
                }
            }
            else if (keyTypeSerializability == Serializability.PrimitiveObject && valueTypeSerializability == Serializability.Object)
            {
                SerializationManager serializationManager = SerializationManager.Instance;
                IPrimitiveObjectSerializer keyPrimitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(keyType);
                KeyValuePair<TKey, TValue>[] keyValuePairs = this.ToArray();
                
                for (int i = 0; i < Count; i++)
                {
                    XElement serializedKeyValuePair = new XElement($"KeyValuePair[{i}]");
                    KeyValuePair<TKey, TValue> keyValuePair = keyValuePairs[i];
                    
                    keyPrimitiveObjectSerializer.Serialize("Key", keyValuePair.Key, out XElement serializedKey);
                    ((ISerializableObject)keyValuePair.Value).Serialize("Value", out XElement serializedValue);
                    
                    serializedKeyValuePair.Add(serializedKey);
                    serializedKeyValuePair.Add(serializedValue);
                    serializedDictionary.Add(serializedKeyValuePair);
                }
            }
            else if (keyTypeSerializability == Serializability.Object && valueTypeSerializability == Serializability.PrimitiveObject)
            {
                SerializationManager serializationManager = SerializationManager.Instance;
                IPrimitiveObjectSerializer valuePrimitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(valueType);
                KeyValuePair<TKey, TValue>[] keyValuePairs = this.ToArray();
                
                for (int i = 0; i < Count; i++)
                {
                    XElement serializedKeyValuePair = new XElement($"KeyValuePair[{i}]");
                    KeyValuePair<TKey, TValue> keyValuePair = keyValuePairs[i];
                    
                    ((ISerializableObject)keyValuePair.Key).Serialize("Key", out XElement serializedKey);
                    valuePrimitiveObjectSerializer.Serialize("Value", keyValuePair.Value, out XElement serializedValue);
                    
                    serializedKeyValuePair.Add(serializedKey);
                    serializedKeyValuePair.Add(serializedValue);
                    serializedDictionary.Add(serializedKeyValuePair);
                }
            }
            else
            {
                KeyValuePair<TKey, TValue>[] keyValuePairs = this.ToArray();
                
                for (int i = 0; i < Count; i++)
                {
                    XElement serializedKeyValuePair = new XElement($"KeyValuePair[{i}]");
                    KeyValuePair<TKey, TValue> keyValuePair = keyValuePairs[i];
                    
                    ((ISerializableObject)keyValuePair.Key).Serialize("Key", out XElement serializedKey);
                    ((ISerializableObject)keyValuePair.Value).Serialize("Value", out XElement serializedValue);
                    
                    serializedKeyValuePair.Add(serializedKey);
                    serializedKeyValuePair.Add(serializedValue);
                    serializedDictionary.Add(serializedKeyValuePair);
                }
            }
        }

        public void Deserialize(XElement serializedDictionary)
        {
            Clear();
            if (keyTypeSerializability == Serializability.PrimitiveObject && valueTypeSerializability == Serializability.PrimitiveObject)
            {
                SerializationManager serializationManager = SerializationManager.Instance;
                IPrimitiveObjectSerializer keyPrimitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(keyType);
                IPrimitiveObjectSerializer valuePrimitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(valueType);
                XElement[] serializedDictionaryChildElements = serializedDictionary.Elements().ToArray();
                
                for (int i = 0; i < serializedDictionaryChildElements.Length; i++)
                {
                    XElement serializedKeyValuePair = serializedDictionaryChildElements[i];
                    XElement serializedKey = serializedKeyValuePair.Element("Key");
                    XElement serializedValue = serializedKeyValuePair.Element("Value");
                    
                    keyPrimitiveObjectSerializer.Deserialize(serializedKey, out object key);
                    valuePrimitiveObjectSerializer.Deserialize(serializedValue, out object value);

                    Add((TKey)key, (TValue)value);
                }
            }
            else if (keyTypeSerializability == Serializability.PrimitiveObject && valueTypeSerializability == Serializability.Object)
            {
                SerializationManager serializationManager = SerializationManager.Instance;
                IPrimitiveObjectSerializer keyPrimitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(keyType);
                XElement[] serializedDictionaryChildElements = serializedDictionary.Elements().ToArray();
                
                for (int i = 0; i < serializedDictionaryChildElements.Length; i++)
                {
                    XElement serializedKeyValuePair = serializedDictionaryChildElements[i];
                    XElement serializedKey = serializedKeyValuePair.Element("Key");
                    XElement serializedValue = serializedKeyValuePair.Element("Value");
                    
                    keyPrimitiveObjectSerializer.Deserialize(serializedKey, out object key);
                    ISerializableObject value = (ISerializableObject)new TValue();
                    value.Deserialize(serializedValue);

                    Add((TKey)key, (TValue)value);
                }
            }
            else if (keyTypeSerializability == Serializability.Object && valueTypeSerializability == Serializability.PrimitiveObject)
            {
                SerializationManager serializationManager = SerializationManager.Instance;
                IPrimitiveObjectSerializer valuePrimitiveObjectSerializer = serializationManager.GetPrimitiveObjectSerializer(valueType);
                XElement[] serializedDictionaryChildElements = serializedDictionary.Elements().ToArray();
                
                for (int i = 0; i < serializedDictionaryChildElements.Length; i++)
                {
                    XElement serializedKeyValuePair = serializedDictionaryChildElements[i];
                    XElement serializedKey = serializedKeyValuePair.Element("Key");
                    XElement serializedValue = serializedKeyValuePair.Element("Value");

                    ISerializableObject key = (ISerializableObject)new TKey();
                    key.Deserialize(serializedKey);
                    valuePrimitiveObjectSerializer.Deserialize(serializedValue, out object value);

                    Add((TKey)key, (TValue)value);
                }
            }
            else
            {
                XElement[] serializedDictionaryChildElements = serializedDictionary.Elements().ToArray();
                
                for (int i = 0; i < serializedDictionaryChildElements.Length; i++)
                {
                    XElement serializedKeyValuePair = serializedDictionaryChildElements[i];
                    XElement serializedKey = serializedKeyValuePair.Element("Key");
                    XElement serializedValue = serializedKeyValuePair.Element("Value");

                    ISerializableObject key = (ISerializableObject)new TKey();
                    key.Deserialize(serializedKey);
                    ISerializableObject value = (ISerializableObject)new TValue();
                    value.Deserialize(serializedValue);

                    Add((TKey)key, (TValue)value);
                }
            }
        }
        #endregion
    }
}
