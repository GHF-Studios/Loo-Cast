using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface ICSharpInstanceType : IInstanceType
    {
        #region Properties
        public ICSharpInstanceType ParentCSharpInstanceType { get; }
        public List<ICSharpInstanceType> ChildCSharpInstanceTypes { get; }
        #endregion
    }
}
