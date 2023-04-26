using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface IInstanceMetaData : IMetaData
    {
        #region Properties
        public IInstanceIdentifier InstanceIdentifier { get; }
        public IType Type { get; }
        #endregion
    }
}
