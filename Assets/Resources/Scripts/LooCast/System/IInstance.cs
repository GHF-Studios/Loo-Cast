using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    using LooCast.System.Identifiers;
    
    public interface IInstance : ISerializableEngineObject
    {
        #region Properties
        IInstanceMetaData InstanceMetaData { get; set; }
        IInstanceData InstanceData { get; set; }

        IInstanceIdentifier InstanceIdentifier { get; }
        ITypeIdentifier InstanceTypeIdentifier { get; }
        #endregion
    }
}
