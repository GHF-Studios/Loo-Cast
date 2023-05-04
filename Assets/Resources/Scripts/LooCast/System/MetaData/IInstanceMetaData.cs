using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    using LooCast.System.Types;

    public interface IInstanceMetaData : IMetaData
    {
        #region Properties
        public IInstanceIdentifier InstanceIdentifier { get; }
        
        public ITypeMetaData InstanceTypeMetaData { get; }
        
        public IInstanceMetaData InstanceMetaDataParent { get; }
        public IEnumerable<IInstanceMetaData> InstancesMetaDataChildren { get; }

        public IType InstanceType { get; }
        
        public IInstance InstanceParent { get; }
        public IEnumerable<IInstance> InstanceChildren { get; }
        #endregion
    }
}
