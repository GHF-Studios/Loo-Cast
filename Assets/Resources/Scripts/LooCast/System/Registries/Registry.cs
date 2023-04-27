using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace LooCast.System.Registry
{
    using LooCast.System.Registries;
    
    public abstract class Registry<KeyType, ValueType> : SystemObject, IRegistry, IEnumerable<KeyValuePair<KeyType, ValueType>>
        where KeyType : Identifier 
        where ValueType : ILooCastObject
    {
        #region Properties
#nullable enable
        public IRegistry? BaseRegistry { get; private set; }
#nullable disable
        public IType RegistryKeyType { get; private set; }
        public IType RegistryValueType { get; private set; }
        public ICollection<KeyType> Keys => dictionary.Keys;
        public ICollection<ValueType> Values => dictionary.Values;
        public int Count => dictionary.Count;
        #endregion

        #region Fields
        private Dictionary<KeyType, ValueType> dictionary;
        #endregion

        #region Methods
        public void Add(Identifier key, ILooCastObject value)
        {
            if (!(key is KeyType))
            {
                throw new global::System.Exception($"Key type {key.GetType()} is not of type {typeof(KeyType)}");
            }
            if (!(value is ValueType))
            {
                throw new global::System.Exception($"Value type {value.GetType()} is not of type {typeof(ValueType)}");
            }
            
            Add((KeyType)key, (ValueType)value);
        }

        public bool Remove(Identifier key)
        {
            if (!(key is KeyType))
            {
                throw new global::System.Exception($"Key type {key.GetType()} is not of type {typeof(KeyType)}");
            }
            
            return Remove((KeyType)key);
        }
        
        public void Add(KeyType key, ValueType value)
        {
            dictionary.Add(key, value);
            BaseRegistry?.Add(key, value);
        }

        public bool ContainsKey(KeyType key)
        {
            return dictionary.ContainsKey(key);
        }

        public bool Remove(KeyType key)
        {
            bool removed = dictionary.Remove(key);
            if (BaseRegistry != null)
            {
                removed &= BaseRegistry.Remove(key);
            }
            return removed;
        }

        public bool TryGetValue(KeyType key, out ValueType value)
        {
            return dictionary.TryGetValue(key, out value);
        }

        public ValueType GetValue(KeyType key)
        {
            if (TryGetValue(key, out ValueType value))
            {
                return value;
            }
            throw new global::System.Exception($"[Registry] Value of type '{typeof(ValueType)}' with key '{key}' not found!");
        }

        public void Add(KeyValuePair<KeyType, ValueType> item)
        {
            Add(item.Key, item.Value);
            BaseRegistry?.Add(item.Key, item.Value);
        }

        public void Clear()
        {
            dictionary.Clear();
            BaseRegistry?.Clear();
        }

        public bool Contains(KeyValuePair<KeyType, ValueType> item)
        {
            return dictionary.Contains(item);
        }

        public bool Remove(KeyValuePair<KeyType, ValueType> item)
        {
            return Remove(item.Key);
        }

        public IEnumerator<KeyValuePair<KeyType, ValueType>> GetEnumerator()
        {
            return dictionary.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return dictionary.GetEnumerator();
        }

#nullable enable
        protected virtual IRegistry? GetBaseRegistry()
        {
            return null;
        }
#nullable disable
        #endregion

        #region Overrides
        protected override void PreConstruct()
        {
            base.PreConstruct();

            BaseRegistry = GetBaseRegistry();
            
            TypeRegistry typeRegistry = MainManager.Instance.MainRegistry.GetRegistry(typeof(IType)) as TypeRegistry;
            
            RegistryKeyType = typeRegistry.GetValue(typeof(KeyType));
            RegistryValueType = typeRegistry.GetValue(typeof(ValueType));
        }
        #endregion
    }
}
