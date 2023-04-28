﻿using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class Data : IData
    {
        #region Properties
        public abstract IData ParentData { get; }
        public abstract IEnumerable<IData> ChildData { get; }

        public abstract ILooCastObject Parent { get; }
        public abstract IEnumerable<ILooCastObject> Children { get; }
        #endregion

        #region Fields
        public abstract bool Validate();
        #endregion
    }
}
