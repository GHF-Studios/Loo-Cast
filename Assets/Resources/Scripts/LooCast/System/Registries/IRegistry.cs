﻿using System.Collections.Generic;

namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    
    public interface IRegistry : IEngineObject
    {
        #region Properties
        public IRegistryIdentifier RegistryIdentifier { get; }
        #endregion

        #region Methods
        public void Add(IObjectIdentifier key, IIdentifiableObject value);
        public bool Remove(IObjectIdentifier key);
        public IIdentifiableObject Get(IObjectIdentifier key);
        public bool ContainsKey(IObjectIdentifier key);
        public bool ContainsValue(IIdentifiableObject value);
        public void Clear();
        #endregion
    }
}
