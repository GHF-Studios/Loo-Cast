using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    
    public interface IInstance : ILooCastObject
    {
        #region Properties
        public IInstanceMetaData InstanceMetaData { get; set; }
        public IInstanceData InstanceData { get; set; }
        public IInstance ParentInstance { get; }
        public IEnumerable<IInstance> ChildInstances { get; }
        public IType InstanceType { get; set; }
        #endregion
    }
}
