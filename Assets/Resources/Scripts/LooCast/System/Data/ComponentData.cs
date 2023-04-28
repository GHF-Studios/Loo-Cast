using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.Data
{
    using LooCast.System.Types;

    public abstract class ComponentData : InstanceData, IComponentData
    {
        #region Properties
        public abstract IGameObjectData ParentGameObjectData { get; }

        public abstract IGameObjectType.IGameObject ParentGameObject { get; }

        public abstract Component UnityEngineComponent { get; }
        #endregion
    }
}
