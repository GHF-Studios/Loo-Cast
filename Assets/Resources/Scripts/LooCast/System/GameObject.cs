using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;

    public class GameObject : IIdentifiable, IDisposable
    {
        #region Properties
        public Identifier Identifier => gameObjectIdentifier;
        public GameObjectIdentifier GameObjectIdentifier => gameObjectIdentifier;

        public Guid GameObjectInstanceGUID => gameObjectInstanceGUID;
        public UnityEngine.GameObject GameObjectInstance => gameObjectInstance;
        public UnityEngine.Component CoreComponentInstance => coreComponentInstance;
        
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
        private UnityEngine.Component coreComponentInstance;

        private Type gameObjectType;
#nullable enable
        private GameObject? parentGameObject;
#nullable disable
        private HashSet<GameObject> childGameObjects;
        private HashSet<Component> containedComponents;

        private bool disposed = false;
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
            coreComponentInstance = gameObjectInstance.AddComponent<ExtendedMonoBehaviour>();
        }
#nullable disable
        #endregion

        #region Finalizer
        ~GameObject()
        {
            Dispose(false);
        }
        #endregion

        #region Methods
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        private void Dispose(bool disposing)
        {
            if (!disposed)
            {
                if (disposing)
                {
                    // Dispose managed resources here, if any.
                    foreach (Component component in containedComponents)
                    {
                        component.Dispose();
                    }
                    containedComponents.Clear();

                    foreach (GameObject childGameObject in childGameObjects)
                    {
                        childGameObject.Dispose();
                    }
                    childGameObjects.Clear();

                    UnityEngine.Object.Destroy(gameObjectInstance);
                }

                // Dispose unmanaged resources here, if any.
                disposed = true;
            }
        }
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
