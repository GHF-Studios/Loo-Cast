using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;

    public class Registry<KeyType, ValueType> : IRegistry where KeyType : IIdentifier where ValueType : IIdentifiable
    {
        #region Properties
        public IRegistryIdentifier RegistryIdentifier => registryIdentifier;
        public IIdentifier Identifier => RegistryIdentifier;
        public Dictionary<KeyType, ValueType> RegistryDictionary => registryDictionary;
        #endregion

        #region Fields
        protected IRegistryIdentifier registryIdentifier;
        protected Dictionary<KeyType, ValueType> registryDictionary;
        #endregion

        #region Constructors
        public Registry(IType keyType, IType valueType)
        {
            if (!ValidateKeyType(keyType))
            {
                throw new Exception($"[Registry] Key type '{keyType.TypeIdentifier}' is not a subclass of '{typeof(KeyType).Name}'!");
            }
            if (!ValidateValueType(valueType))
            {
                throw new Exception($"[Registry] Value type '{valueType.TypeIdentifier}' is not a subclass of '{typeof(ValueType).Name}'!");
            }

            registryIdentifier = new RegistryIdentifier(keyType.TypeIdentifier.TypeID, valueType.TypeIdentifier.TypeID);
            registryDictionary = new Dictionary<KeyType, ValueType>();
        }
        #endregion

        #region Operators
        public ValueType this[KeyType key]
        {
            get
            {
                return registryDictionary[key];
            }
            set
            {
                registryDictionary[key] = value;
            }
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

        public virtual bool ValidateKeyType(IType keyType)
        {
            return keyType.CSSystemType.IsSubclassOf(typeof(KeyType));
        }

        public virtual bool ValidateValueType(IType valueType)
        {
            return valueType.CSSystemType.IsSubclassOf(typeof(ValueType));
        }
        #endregion
    }
}
