using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IGameObjectType : IUnityInstanceType
    {
        #region Properties
        public IGameObjectType ParentGameObjectType { get; }
        public List<IGameObjectType> ChildGameObjectTypes { get; }
        #endregion
    }
}
