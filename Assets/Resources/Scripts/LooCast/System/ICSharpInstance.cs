using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public interface ICSharpInstance : IInstance, ICSharpInstanceIdentifiable
    {
        #region Properties
        public ICSharpInstanceType CSharpInstanceType { get; }
        public object SystemObject { get; }
        public ICSharpInstance ParentCSharpInstance { get; }
        public List<ICSharpInstance> ChildCSharpInstances { get; }
        #endregion
    }
}