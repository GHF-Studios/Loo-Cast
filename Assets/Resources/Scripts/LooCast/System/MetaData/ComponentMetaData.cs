﻿using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    [Serializable]
    public abstract class ComponentMetaData : InstanceMetaData, IComponentMetaData
    {
        #region Properties
        public abstract Component UnityEngineComponent { get; }
        public abstract IComponentIdentifier ComponentIdentifier { get; }
        public abstract IComponentTypeMetaData ComponentTypeMetaData { get; }
        public abstract IGameObjectMetaData ParentGameObjectMetaData { get; }
        #endregion
    }
}
