using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IObjectType : IInstanceType
    {
        #region Properties
        public IObjectType ParentObjectType { get; }
        public List<IObjectType> ChildObjectTypes { get; }
        #endregion
    }
}
