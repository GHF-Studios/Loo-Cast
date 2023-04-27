using LooCast.System.Data;
using LooCast.System.MetaData;
using System;

namespace LooCast.System.Types
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;

    public abstract class InstanceType<TInstance> : Type<TInstance>, IInstanceType
        where TInstance : IInstanceType.IInstance, new()
    {
        #region Properties
        public abstract IInstanceTypeMetaData InstanceTypeMetaData { get; set; }
        
        public abstract IInstanceTypeData InstanceTypeData { get; set; }
        #endregion
    }
}
