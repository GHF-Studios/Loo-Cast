using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;
    
    public class Registry<KeyType, ValueType> : IGenericIdentifiable<Registry<IIdentifier, IIdentifiable>> where KeyType : IIdentifier where ValueType : IIdentifiable
    {
        #region Properties
        public RegistryIdentifier RegistryIdentifier => registryIdentifier;
        public IIdentifier Identifier => RegistryIdentifier;
        public Dictionary<KeyType, ValueType> RegistryDictionary => registryDictionary;
        #endregion

        #region Fields
        private RegistryIdentifier registryIdentifier;
        private Dictionary<KeyType, ValueType> registryDictionary;
        #endregion

        #region Constructors
        public Registry(RegistryIdentifier registryIdentifier)
        {
            this.registryIdentifier = registryIdentifier;
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
