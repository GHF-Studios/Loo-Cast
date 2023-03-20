using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IObjectType : ICSharpInstanceType
    {
        #region Properties
        public IObjectType ParentObjectType { get; }
        public List<IObjectType> ChildObjectTypes { get; }
        #endregion
    }
}
