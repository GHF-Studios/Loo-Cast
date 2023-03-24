using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

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