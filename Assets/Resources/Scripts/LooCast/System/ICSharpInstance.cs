using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface ICSharpInstance : IInstance, ICSharpInstanceIdentifiable
    {
        #region Properties
        public ICSharpInstanceType CSharpInstanceType { get; }
        public object CSharpInstanceObject { get; }
        public ICSharpInstance ParentCSharpInstance { get; }
        public List<ICSharpInstance> ChildCSharpInstances { get; }
        #endregion
    }
}