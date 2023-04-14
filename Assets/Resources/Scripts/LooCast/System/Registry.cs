using LooCast.System.MetaData;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace LooCast.System
{
    public abstract class Registry<KeyType, ValueType> : SystemObject, IEnumerable<KeyValuePair<KeyType, ValueType>>
        where KeyType : Identifier 
        where ValueType : ILooCastObject
    {
        #region Properties
        public RegistryMetaData RegistryMetaData { get; private set; }
#nullable enable
        public Registry<KeyType, ValueType>? BaseRegistry { get; private set; }

        public ICollection<KeyType> Keys => dictionary.Keys;
        public ICollection<ValueType> Values => dictionary.Values;
        public int Count => dictionary.Count;
#nullable disable
        #endregion

        #region Fields
        private Dictionary<KeyType, ValueType> dictionary;
        #endregion

        #region Methods
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

        public void Add(KeyValuePair<KeyType, ValueType> item)
        {
            Add(item.Key, item.Value);
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
        protected virtual Registry<KeyType, ValueType>? GetBaseRegistry()
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
        }

        protected override void CreateMetaData<SystemObjectType, SystemObjectMetaDataType>(ref SystemObjectMetaDataType systemObjectMetaData)
        {
            base.CreateMetaData<SystemObjectType, SystemObjectMetaDataType>(ref systemObjectMetaData);

            if (!(systemObjectMetaData is RegistryMetaData))
            {
                throw new global::System.Exception("SystemObjectMetaData is not of type RegistryMetaData!");
            }

            RegistryMetaData registryMetaData = (RegistryMetaData)(SystemObjectMetaData)systemObjectMetaData;
        }

        public override void SetMetaData(SystemObjectMetaData systemObjectMetaData)
        {
            base.SetMetaData(systemObjectMetaData);

            if (!(systemObjectMetaData is RegistryMetaData))
            {
                throw new global::System.Exception("SystemObjectMetaData is not of type RegistryMetaData!");
            }
            
            RegistryMetaData = (RegistryMetaData)systemObjectMetaData;
        }
        #endregion
    }
}
