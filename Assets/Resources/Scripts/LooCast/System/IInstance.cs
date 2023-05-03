using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    
    public interface IInstance : ILooCastObject
    {
        #region Properties
        public IType InstanceType { get; }
        
        public IInstanceMetaData InstanceMetaData { get; set; }
        public IInstanceData InstanceData { get; set; }

        public IInstance InstanceParent { get; }
        public IEnumerable<IInstance> InstanceChildren { get; }
        #endregion
    }
}
