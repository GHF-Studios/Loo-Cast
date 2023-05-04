using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    using LooCast.System.Types;

    public abstract class InstanceMetaData : MetaData, IInstanceMetaData
    {
        #region Properties
        public abstract IInstanceIdentifier InstanceIdentifier { get; }
        public abstract IInstanceTypeMetaData InstanceTypeMetaData { get; }
        public abstract IInstanceMetaData InstanceMetaDataParent { get; }
        public abstract IEnumerable<IInstanceMetaData> InstancesMetaDataChildren { get; }

        public abstract IInstanceType InstanceType { get; }
        public abstract IInstanceType.IInstance ParentInstance { get; }
        public abstract IEnumerable<IInstanceType.IInstance> ChildInstances { get; }
        #endregion
    }
}
