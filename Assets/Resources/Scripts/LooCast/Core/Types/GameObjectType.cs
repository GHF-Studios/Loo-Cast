using UnityEngine;
using System.Collections;
using System.Collections.Generic;
using System;

namespace LooCast.Core.Types
{
    using LooCast.Core.Data;
    using LooCast.Core.Identifiers;
    using LooCast.Core.MetaData;
    
    public abstract class GameObjectType<TInstance> : InstanceType<TInstance>, IGameObjectType
        where TInstance : GameObjectType<TInstance>.GameObject, new()
    {
        #region Classes
        public abstract class GameObject : IGameObjectType.IGameObject
        {
            #region Properties
            public abstract IMetaData MetaData { get; set; }
            public abstract IInstanceMetaData InstanceMetaData { get; set; }
            public abstract IGameObjectMetaData GameObjectMetaData { get; set; }

            public abstract IData Data { get; set; }
            public abstract IInstanceData InstanceData { get; set; }
            public abstract IGameObjectData GameObjectData { get; set; }
            #endregion

            #region Static Methods
#nullable enable
            public static GameObjectType CreateGameObject<GameObjectType, GameObjectMetaDataType>(GameObjectMetaDataType? gameObjectMetaData = default(GameObjectMetaDataType))
                where GameObjectType : GameObject, new()
                where GameObjectMetaDataType : GameObjectMetaData, new()
            {
                if (gameObjectMetaData == null)
                {
                    gameObjectMetaData = Activator.CreateInstance<GameObjectMetaDataType>();
                }

                GameObjectType gameObject = Activator.CreateInstance<GameObjectType>();
                
                gameObject.CreateMetaData<GameObjectType, GameObjectMetaDataType>(ref gameObjectMetaData);
                
                gameObject.SetMetaData(gameObjectMetaData);
                
                gameObject.PreConstruct();
                gameObject.Construct();
                gameObject.PostConstruct();
                
                return gameObject;
            }
#nullable disable
            #endregion

            #region Methods
            public abstract bool Validate();

            protected virtual void CreateMetaData<GameObjectType, GameObjectMetaDataType>(ref GameObjectMetaDataType gameObjectMetaData)
                where GameObjectType : UnityEngine.GameObject, new()
                where GameObjectMetaDataType : GameObjectMetaData, new()
            {
                gameObjectMetaData.GameObjectIdentifier = new GameObjectIdentifier(TypeManager.Instance.GetType<GameObjectType>().TypeIdentifier, Guid.NewGuid());
                gameObjectMetaData.ParentGameObject = null;
                gameObjectMetaData.ChildGameObjects = new List<UnityEngine.GameObject>();
                gameObjectMetaData.ContainedComponents = new List<Component>();
            }

            public virtual void SetMetaData(GameObjectMetaData gameObjectMetaData)
            {
                this.gameObjectMetaData = gameObjectMetaData;
            }

            protected virtual void PreConstruct()
            {
                unityEngineGameObject = new UnityEngine.GameObject();
            }

            protected virtual void Construct()
            {

            }

            protected virtual void PostConstruct()
            {

            }
            #endregion
        }
        #endregion

        #region Properties
        public abstract IGameObjectTypeMetaData GameObjectTypeMetaData { get; set; }

        public abstract IGameObjectTypeData GameObjectTypeData { get; set; }
        #endregion
    }
}
