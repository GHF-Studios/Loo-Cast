﻿using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface ISerializableEngineObject : IEngineObject, IHierarchicalObject
    {
        #region Properties
        public IMetaData MetaData { get; set; }
        public IData Data { get; set; }
        #endregion
    }
}
