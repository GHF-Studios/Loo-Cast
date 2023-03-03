using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IObjectType : ICSharpInstanceType
    {
        #region Properties
        public IObjectType ParentObjectType { get; }
        public List<IObjectType> ChildObjectTypes { get; }
        #endregion
    }
}
