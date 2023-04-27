using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    public abstract class InstanceMetaData : MetaData, IInstanceMetaData
    {
        #region Properties
        public abstract IInstanceIdentifier InstanceIdentifier { get; }
        public abstract ITypeMetaData TypeMetaData { get; }
        public abstract IInstanceMetaData ParentInstanceMetaData { get; }
        public abstract IEnumerable<IInstanceMetaData> ChildInstancesMetaData { get; }
        #endregion
    }
}
