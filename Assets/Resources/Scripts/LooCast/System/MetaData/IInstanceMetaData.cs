using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface IInstanceMetaData : IMetaData
    {
        #region Properties
        public IInstanceIdentifier InstanceIdentifier { get; }
        public ITypeMetaData TypeMetaData { get; }
        public IInstanceMetaData ParentInstanceMetaData { get; }
        public IEnumerable<IInstanceMetaData> ChildInstancesMetaData { get; }
        #endregion
    }
}
