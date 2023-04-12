using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;

    public class GameObject : ILooCastObject
    {
        #region Properties
        public Identifier Identifier => gameObjectIdentifier;
        public GameObjectIdentifier GameObjectIdentifier => gameObjectIdentifier;
        public Guid GameObjectInstanceGUID => gameObjectInstanceGUID;
        public UnityEngine.GameObject UnityEngineGameObject => unityEngineGameObject;
        public Type GameObjectType => gameObjectType;
#nullable enable 
        public GameObject? ParentGameObject => parentGameObject;
#nullable disable
        public HashSet<GameObject> ChildGameObjects => childGameObjects;
        public HashSet<Component> ContainedComponents => containedComponents;
        #endregion

        #region Fields
        private GameObjectIdentifier gameObjectIdentifier;
        private Guid gameObjectInstanceGUID;
        private UnityEngine.GameObject unityEngineGameObject;
        private Type gameObjectType;
#nullable enable
        private GameObject? parentGameObject;
#nullable disable
        private HashSet<GameObject> childGameObjects;
        private HashSet<Component> containedComponents;
        #endregion

        #region Static Methods
#nullable enable
        public static T Create<T>(GameObject? parentGameObject = null) where T : GameObject
        {
            T gameObject = Activator.CreateInstance<T>();
            gameObject.unityEngineGameObject = new UnityEngine.GameObject();
            gameObject.gameObjectInstanceGUID = Guid.NewGuid();
            gameObject.gameObjectType = new Type<T>();
            gameObject.gameObjectIdentifier = new GameObjectIdentifier(gameObject.gameObjectType.TypeIdentifier, gameObject.gameObjectInstanceGUID);
            gameObject.parentGameObject = parentGameObject;
            gameObject.childGameObjects = new HashSet<GameObject>();
            gameObject.containedComponents = new HashSet<Component>();
            gameObject.PreConstruct();
            gameObject.Construct();
            gameObject.PostConstruct();
            return gameObject;
        }
#nullable disable
        #endregion

        #region Methods
        protected virtual void PreConstruct()
        {

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
