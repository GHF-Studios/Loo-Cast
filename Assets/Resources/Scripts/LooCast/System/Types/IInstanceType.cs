using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    
    public interface IInstanceType : IType
    {
        #region Interfaces
        public interface IInstance : ILooCastObject
        {
            public IInstanceMetaData InstanceMetaData { get; set; }

            public IInstanceData InstanceData { get; set; }
        }
        #endregion
        
        #region Properties
        public IInstanceTypeMetaData InstanceTypeMetaData { get; set; }
        
        public IInstanceTypeData InstanceTypeData { get; set; }
        #endregion
    }
}
