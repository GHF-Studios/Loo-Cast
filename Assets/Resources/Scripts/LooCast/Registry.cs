using System;
using System.Collections.Generic;

namespace LooCast
{
    public sealed class Registry<KeyType, ValueType> : IIdentifiable where KeyType : IIdentifier where ValueType : IIdentifiable
    {
        #region Properties
        public TypeIdentifier RegistryTypeIdentifier => registryTypeIdentifier;
        public IIdentifier Identifier => RegistryTypeIdentifier;
        public Dictionary<KeyType, ValueType> RegistryDictionary => registryDictionary;
        #endregion

        #region Fields
        private TypeIdentifier registryTypeIdentifier;
        private Dictionary<KeyType, ValueType> registryDictionary;
        #endregion

        #region Constructors
        public Registry(TypeIdentifier registryTypeIdentifier)
        {
            this.registryTypeIdentifier = registryTypeIdentifier;
            registryDictionary = new Dictionary<KeyType, ValueType>();
        }
        #endregion

        #region Methods
        public void Register(KeyType key, ValueType value)
        {
            registryDictionary.Add(key, value);
        }

        public void Unregister(KeyType key)
        {
            registryDictionary.Remove(key);
        }

        public ValueType Get(KeyType key)
        {
            return registryDictionary[key];
        }
        #endregion
    }
}
