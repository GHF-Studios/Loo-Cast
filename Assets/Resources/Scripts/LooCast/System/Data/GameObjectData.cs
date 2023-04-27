using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.Data
{
    public abstract class GameObjectData : IInstanceData
    {
        #region Properties
        public abstract IInstanceData ParentInstanceData { get; }
        public abstract IEnumerable<IInstanceData> ChildInstanceData { get; }
        public abstract IData ParentData { get; }
        public abstract IEnumerable<IData> ChildData { get; }

        public abstract GameObject UnityEngineGameObject { get; }
        public abstract IEnumerable<Component> ContainedComponents { get; }
        #endregion

        #region Fields
        public abstract bool Validate();
        #endregion
    }
}
