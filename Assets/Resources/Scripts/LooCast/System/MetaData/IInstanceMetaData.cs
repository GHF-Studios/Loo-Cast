using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    using LooCast.System.Types;

    public interface IInstanceMetaData : IMetaData
    {
        #region Properties
        public IInstanceIdentifier InstanceIdentifier { get; }
        public IInstanceTypeMetaData InstanceTypeMetaData { get; }
        public IInstanceMetaData ParentInstanceMetaData { get; }
        public IEnumerable<IInstanceMetaData> ChildInstancesMetaData { get; }

        public IInstanceType InstanceType { get; }
        public IInstanceType.IInstance ParentInstance { get; }
        public IEnumerable<IInstanceType.IInstance> ChildInstances { get; }
        #endregion
    }
}
