using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.MetaData;

    public abstract class GameObject : ILooCastObject
    {
        #region Properties
        public Identifier Identifier => gameObjectMetaData.GameObjectIdentifier;
        public GameObjectMetaData GameObjectMetaData => gameObjectMetaData;
        public UnityEngine.GameObject UnityEngineGameObject => unityEngineGameObject;
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
            GameObjectType gameObject = Activator.CreateInstance<GameObjectType>();
            if (gameObjectMetaData == null)
            {
                gameObjectMetaData = Activator.CreateInstance<GameObjectMetaDataType>();
                gameObject.CreateMetaData<GameObjectType, GameObjectMetaDataType>(ref gameObjectMetaData);
            }
            gameObject.SetMetaData(gameObjectMetaData);
            gameObject.PreConstruct();
            gameObject.Construct();
            gameObject.PostConstruct();
            return gameObject;
        }
#nullable disable
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
}
