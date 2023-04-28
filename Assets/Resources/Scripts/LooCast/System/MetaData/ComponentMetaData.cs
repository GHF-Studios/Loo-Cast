﻿using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;
    using LooCast.System.Types;

    [Serializable]
    public abstract class ComponentMetaData : InstanceMetaData, IComponentMetaData
    {
        #region Properties
        public abstract IComponentIdentifier ComponentIdentifier { get; }
        public abstract IComponentTypeMetaData ComponentTypeMetaData { get; }
        public abstract IGameObjectMetaData ParentGameObjectMetaData { get; }

        public IComponentType ComponentType { get; private set; }
        public IGameObjectType.IGameObject ParentGameObject { get; private set; }
        #endregion

        #region Methods
        public override void PreInitialize()
        {
            base.PreInitialize();
            
            ComponentType = MainManager.Instance.MainRegistry.TypeRegistry.GetValue(ComponentTypeMetaData.TypeIdentifier) as IComponentType;
            ParentGameObject = MainManager.Instance.MainRegistry.GameObjectRegistry.GetValue(ParentGameObjectMetaData.GameObjectIdentifier;
        }
        #endregion
    }
}
