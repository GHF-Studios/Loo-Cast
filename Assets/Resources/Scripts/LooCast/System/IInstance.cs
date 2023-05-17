using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    
    public interface IInstance : IObject
    {
        #region Properties
        public IInstanceMetaData InstanceMetaData { get; set; }
        public IInstanceData InstanceData { get; set; }
        #endregion
    }
}
