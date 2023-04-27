using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface IComponentTypeMetaData : IInstanceTypeMetaData
    {
        #region Properties
        public IGameObjectTypeMetaData ParentGameObjectTypeMetaData { get; }
        #endregion
    }
}
