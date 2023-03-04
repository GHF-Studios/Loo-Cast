﻿using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IObjectDataType : ICSharpInstanceDataType
    {
        #region Properties
        public IObjectDataType ParentObjectDataType { get; }
        public List<IObjectDataType> ChildObjectDataTypes { get; }
        #endregion
    }
}
