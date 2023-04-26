using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;

namespace LooCast.System.Types
{
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;

    public abstract class GameObjectType<TInstance> : Type<TInstance>
        where TInstance : GameObjectType<TInstance>.Instance, new()
    {
        #region Classes
        public abstract class Instance : IType.IInstance
        {
            #region Properties
            public IMetaData MetaData => InstanceMetaData;
            public IInstanceMetaData InstanceMetaData => GameObjectMetaData;
            public GameObjectMetaData GameObjectMetaData
            {
                get
                {

                }

                set
                {

                }
            }

            public IData Data => InstanceData;
            public IInstanceData InstanceData => GameObjectData;
            public GameObjectData GameObjectData
            {
                get
                {

                }

                set
                {

                }
            }

            public GameObject UnityEngineGameObject => unityEngineGameObject;
            public List<Component> ContainedComponents => gameObjectMetaData.ContainedComponents;
            #endregion

            #region Fields
            private GameObjectMetaData gameObjectMetaData;
            private UnityEngine.GameObject unityEngineGameObject;
            #endregion

            #region Static Methods
#nullable enable
            public static GameObjectType CreateGameObject<GameObjectType, GameObjectMetaDataType>(GameObjectMetaDataType? gameObjectMetaData = default(GameObjectMetaDataType))
                where GameObjectType : GameObject, new()
                where GameObjectMetaDataType : GameObjectMetaData, new()
            {
                if (gameObjectMetaData == null)
                {
                    return CreateGameObject<GameObjectType>();
                }

                GameObjectType gameObject = Activator.CreateInstance<GameObjectType>();
                gameObjectMetaData = Activator.CreateInstance<GameObjectMetaDataType>();
                gameObject.CreateMetaData<GameObjectType, GameObjectMetaDataType>(ref gameObjectMetaData);
                gameObject.SetMetaData(gameObjectMetaData);
                gameObject.PreConstruct();
                gameObject.Construct();
                gameObject.PostConstruct();
                return gameObject;
            }
#nullable disable

            public static GameObjectType CreateGameObject<GameObjectType>()
                where GameObjectType : GameObject, new()
            {
                GameObjectType gameObject = Activator.CreateInstance<GameObjectType>();
                GameObjectMetaData gameObjectMetaData = new GameObjectMetaData();
                gameObject.CreateMetaData<GameObjectType, GameObjectMetaData>(ref gameObjectMetaData);
                gameObject.SetMetaData(gameObjectMetaData);
                gameObject.PreConstruct();
                gameObject.Construct();
                gameObject.PostConstruct();
                return gameObject;
            }
            #endregion

            #region Methods
            protected virtual void CreateMetaData<GameObjectType, GameObjectMetaDataType>(ref GameObjectMetaDataType gameObjectMetaData)
                where GameObjectType : GameObject, new()
                where GameObjectMetaDataType : GameObjectMetaData, new()
            {
                gameObjectMetaData.GameObjectIdentifier = new GameObjectIdentifier(TypeManager.Instance.GetType<GameObjectType>().TypeIdentifier, Guid.NewGuid());
                gameObjectMetaData.ParentGameObject = null;
                gameObjectMetaData.ChildGameObjects = new List<GameObject>();
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

        #region Constructors
        #endregion
    }
}
