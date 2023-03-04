﻿using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IGameObjectMetaDataType : IUnityInstanceMetaDataType
    {
        #region Properties
        public IGameObjectMetaDataType ParentGameObjectMetaDataType { get; }
        public List<IGameObjectMetaDataType> ChildGameObjectMetaDataTypes { get; }
        #endregion
    }
}
