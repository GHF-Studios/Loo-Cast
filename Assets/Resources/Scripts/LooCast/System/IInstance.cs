using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    using LooCast.System.Identifiers;
    
    public interface IInstance : ISerializableEngineObject
    {
        #region Properties
        public IInstanceIdentifier InstanceIdentifier { get; }
        public ITypeIdentifier InstanceTypeIdentifier { get; }
        public IInstanceMetaData InstanceMetaData { get; set; }
        public IInstanceData InstanceData { get; set; }
        #endregion
    }
}
