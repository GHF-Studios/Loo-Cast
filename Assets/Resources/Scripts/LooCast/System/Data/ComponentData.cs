using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.Data
{
    public abstract class ComponentData : IInstanceData
    {
        #region Properties
        public abstract IInstanceData ParentInstanceData { get; }
        public abstract IEnumerable<IInstanceData> ChildInstanceData { get; }
        public abstract IData ParentData { get; }
        public abstract IEnumerable<IData> ChildData { get; }

        public abstract Component UnityEngineComponent { get; }
        public abstract GameObject ContainingGameObject { get; }
        #endregion

        #region Fields
        public abstract bool Validate();
        #endregion
    }
}
