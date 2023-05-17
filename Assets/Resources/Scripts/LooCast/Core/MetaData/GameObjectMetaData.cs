using System;
using System.Linq;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Core.MetaData
{
    using LooCast.Core.Identifiers;
    using LooCast.Core.Types;

    [Serializable]
    public abstract class GameObjectMetaData : InstanceMetaData, IGameObjectMetaData
    {
        #region Properties
        public abstract IGameObjectIdentifier GameObjectIdentifier { get; }
        public abstract IGameObjectTypeMetaData GameObjectTypeMetaData { get; }
        public abstract IGameObjectMetaData ParentGameObjectMetaData { get; }
        public abstract IEnumerable<IGameObjectMetaData> ChildGameObjectsMetaData { get; }
        public abstract IEnumerable<IComponentMetaData> ChildComponentsMetaData { get; }

        public IGameObjectType GameObjectType { get; private set; }
        public IGameObjectType.IGameObject ParentGameObject { get; private set; }
        public IEnumerable<IGameObjectType.IGameObject> ChildGameObjects { get; private set; }
        public IEnumerable<IComponentType.IComponent> ChildComponents { get; private set; }
        #endregion

        #region Methods
        public override void PreInitialize()
        {
            base.PreInitialize();
            
            GameObjectType = MainManager.Instance.MainRegistry.TypeRegistry.GetValue(GameObjectTypeMetaData.TypeIdentifier) as IComponentType;
            ParentGameObject = MainManager.Instance.MainRegistry.GameObjectRegistry.GetValue(ParentGameObjectMetaData.GameObjectIdentifier);
            ChildGameObjects = MainManager.Instance.MainRegistry.GameObjectRegistry.GetValues(ChildGameObjectsMetaData.Select(x => x.GameObjectIdentifier));
            ChildComponents = MainManager.Instance.MainRegistry.ComponentRegistry.GetValues(ChildComponentsMetaData.Select(x => x.ComponentIdentifier));
        }
        #endregion
    }
}
