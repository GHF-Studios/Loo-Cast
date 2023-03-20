using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IInstanceType : IType
    {
        #region Properties
        public IInstanceType ParentInstanceType { get; }
        public List<IInstanceType> ChildInstanceTypes { get; }
        #endregion
    }
}
