using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;

    public class GameObject : IHierarchyElement
    {
        #region Properties
        public Identifier Identifier => gameObjectIdentifier;
        public GameObjectIdentifier GameObjectIdentifier => gameObjectIdentifier;

        public Guid GameObjectInstanceGUID => gameObjectInstanceGUID;
        public UnityEngine.GameObject GameObjectInstance => gameObjectInstance;
        public UnityEngine.Component GameObjectComponentInstance => gameObjectComponentInstance;
        
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
        private UnityEngine.GameObject gameObjectInstance;
        private UnityEngine.Component gameObjectComponentInstance;

        private Type gameObjectType;
#nullable enable
        private GameObject? parentGameObject;
#nullable disable
        private HashSet<GameObject> childGameObjects;
        private HashSet<Component> containedComponents;
        #endregion

        #region Constructors
#nullable enable
        public GameObject(Type gameObjectType, GameObject? parentGameObject = null)
        {
            this.gameObjectType = gameObjectType;
            this.parentGameObject = parentGameObject;

            childGameObjects = new HashSet<GameObject>();
            containedComponents = new HashSet<Component>();

            gameObjectIdentifier = new GameObjectIdentifier(gameObjectType.TypeIdentifier, Guid.NewGuid());
            gameObjectInstanceGUID = gameObjectIdentifier.GameObjectInstanceGUID;
            gameObjectInstance = new UnityEngine.GameObject();
            gameObjectComponentInstance = gameObjectInstance.AddComponent<ExtendedMonoBehaviour>();
        }
#nullable disable
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is GameObject otherGameObject)
            {
                return Equals(otherGameObject);
            }
            return false;
        }

        public bool Equals(GameObject otherGameObject)
        {
            return GameObjectIdentifier.Equals(otherGameObject.GameObjectIdentifier);
        }

        public override int GetHashCode()
        {
            return GameObjectIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return GameObjectIdentifier.ToString();
        }
        #endregion
    }
}
