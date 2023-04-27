using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public abstract class GameObjectTypeMetaData : IInstanceTypeMetaData
    {
        #region Properties
        public abstract HierarchyElement HierarchyElement { get; }
        
        public abstract TypeIdentifier TypeIdentifier { get; }
        
        public abstract IMetaData ParentMetaData { get; }
        public abstract ITypeMetaData ParentTypeMetaData { get; }
        public abstract IInstanceTypeMetaData ParentInstanceTypeMetaData { get; }
        
        public abstract IEnumerable<IMetaData> ChildMetaData { get; }
        public abstract IEnumerable<ITypeMetaData> ChildTypesMetaData { get; }
        public abstract IEnumerable<IInstanceTypeMetaData> ChildInstanceTypesMetaData { get; }

        public abstract bool Validate();
        #endregion
    }
}
