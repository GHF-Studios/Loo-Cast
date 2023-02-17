using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IUnityInstanceType : IInstanceType
    {
        #region Properties
        public IUnityInstanceType ParentUnityInstanceType { get; }
        public List<IUnityInstanceType> ChildUnityInstanceTypes { get; }
        #endregion
    }
}
