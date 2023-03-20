using System.Collections.Generic;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public interface IRegistry<KeyType, ValueType> : IRegistryIdentifiable where KeyType : IIdentifier where ValueType : IIdentifiable
    {
        #region Properties
        public Dictionary<KeyType, ValueType> RegistryDictionary { get; }
        #endregion

        #region Methods
        public void Register(KeyType key, ValueType value);
        public void Unregister(KeyType key);
        public ValueType Get(KeyType key);
        public bool ValidateKeyType(IType keyType);
        public bool ValidateValueType(IType valueType);
        #endregion
    }
}