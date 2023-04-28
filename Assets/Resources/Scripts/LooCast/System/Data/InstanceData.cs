﻿using LooCast.System.Types;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class InstanceData : Data, IInstanceData
    {
        #region Properties
        public abstract IInstanceData ParentInstanceData { get; }
        public abstract IEnumerable<IInstanceData> ChildInstanceData { get; }

        public abstract IInstanceType.IInstance ParentInstance { get; }
        public abstract IEnumerable<IInstanceType.IInstance> ChildInstances { get; }
        #endregion
    }
}
