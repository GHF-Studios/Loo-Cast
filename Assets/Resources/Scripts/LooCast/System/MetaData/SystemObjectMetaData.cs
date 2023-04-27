using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    [Serializable]
    public abstract class SystemObjectMetaData : InstanceMetaData, ISystemObjectMetaData
    {
        #region Properties
        public abstract ISystemObjectIdentifier SystemObjectIdentifier { get; }
        public abstract ISystemObjectTypeMetaData SystemObjectTypeMetaData { get; }
        public abstract ISystemObjectMetaData ParentSystemObjectMetaData { get; }
        public abstract IEnumerable<ISystemObjectMetaData> ChildSystemObjectsMetaData { get; }
        #endregion
    }
}
