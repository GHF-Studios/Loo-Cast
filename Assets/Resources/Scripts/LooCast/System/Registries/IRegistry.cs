﻿using System.Collections.Generic;

namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    
    public interface IRegistry : IObject, IIdentifiable
    {
        #region Properties
        public IRegistryIdentifier RegistryIdentifier { get; }
        public HierarchyElement RegistryHierarchyElement { get; }

        public IRegistryMetaData RegistryMetaData { get; set; }
        public IRegistryData RegistryData { get; set; }
        #endregion

        #region Methods
        public void Add(IIdentifier key, IIdentifiable value);
        public bool Remove(IIdentifier key);
        public IIdentifiable Get(IIdentifier key);
        public bool ContainsKey(IIdentifier key);
        public bool ContainsValue(IIdentifiable value);
        public void Clear();
        #endregion
    }
}
