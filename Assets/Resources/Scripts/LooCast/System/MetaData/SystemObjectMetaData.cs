using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    [Serializable]
    public abstract class SystemObjectMetaData : IInstanceMetaData
    {
        #region Properties
        public abstract HierarchyElement HierarchyElement { get; }

        public abstract IIdentifier Identifier { get; }
        public abstract IInstanceIdentifier InstanceIdentifier { get; }
        public abstract SystemObjectIdentifier SystemObjectIdentifier { get; }
        
        public abstract IMetaData MetaData { get; }
        public abstract ITypeMetaData TypeMetaData { get; }
        public abstract SystemObjectTypeMetaData SystemObjectTypeMetaData { get; }
        
        public abstract IMetaData ParentMetaData { get; }
        public abstract IInstanceMetaData ParentInstanceMetaData { get; }
        public abstract SystemObjectMetaData ParentSystemObjectMetaData { get; }
        
        public abstract IEnumerable<IMetaData> ChildMetaData { get; }
        public abstract IEnumerable<IInstanceMetaData> ChildInstancesMetaData { get; }
        public abstract IEnumerable<SystemObjectMetaData> ChildSystemObjectsMetaData { get; }
        #endregion
        
        #region Methods
        public abstract bool Validate();
        #endregion
    }
}
