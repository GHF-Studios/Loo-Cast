using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    [Serializable]
    public abstract class GameObjectMetaData : IInstanceMetaData
    {
        #region Properties
        public abstract HierarchyElement HierarchyElement { get; }

        public abstract IIdentifier Identifier { get; }
        public abstract IInstanceIdentifier InstanceIdentifier { get; }
        public abstract GameObjectIdentifier GameObjectIdentifier { get; }

        public abstract IMetaData MetaData { get; }
        public abstract ITypeMetaData TypeMetaData { get; }
        public abstract GameObjectTypeMetaData GameObjectTypeMetaData { get; }

        public abstract IMetaData ParentMetaData { get; }
        public abstract IInstanceMetaData ParentInstanceMetaData { get; }
        public abstract GameObjectMetaData ParentGameObjectMetaData { get; }

        public abstract IEnumerable<IMetaData> ChildMetaData { get; }
        public abstract IEnumerable<IInstanceMetaData> ChildInstancesMetaData { get; }
        public abstract IEnumerable<GameObjectMetaData> ChildGameObjectsMetaData { get; }
        public abstract IEnumerable<ComponentMetaData> ChildComponentsMetaData { get; }
        #endregion

        #region Methods
        public abstract bool Validate();
        #endregion
    }
}
