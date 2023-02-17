using System.Collections.Generic;

namespace LooCast.System
{
    using System.Identification;

    public interface IInstance : IInstanceIdentifiable
    {
        #region Properties
        public IInstanceType InstanceType { get; }
        public object InstanceObject { get; }
        public IInstance ParentInstance { get; }
        public List<IInstance> ChildInstances { get; }
        #endregion
    }
}