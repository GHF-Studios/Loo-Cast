using System;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.Registries;

    public class GameObject : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => gameObjectIdentifier;
        public GameObjectIdentifier GameObjectIdentifier => gameObjectIdentifier;

        public Guid GameObjectInstanceGUID => gameObjectInstanceGUID;
        public UnityEngine.GameObject GameObjectInstance => gameObjectInstance;
        
        public Type ContainingType => containingType;
        
#nullable enable 
        public GameObject? ParentGameObject => parentGameObject;
#nullable disable
        public GameObjectRegistry ChildGameObjects => childGameObjects;
        
        public ComponentRegistry ContainedComponents => containedComponents;
        #endregion

        #region Fields
#nullable enable 
        private GameObjectIdentifier? gameObjectIdentifier;
#nullable disable

        private Guid gameObjectInstanceGUID;
        private UnityEngine.GameObject gameObjectInstance;

        private Type containingType;

#nullable enable
        private GameObject? parentGameObject;
#nullable disable
        private GameObjectRegistry childGameObjects;

        private ComponentRegistry containedComponents;
        #endregion

        #region Constructors
        public GameObject(TypeIdentifier typeIdentifier, GameObject parentGameObject = null)
        {
            TypeManager typeManager = TypeManager.Instance;

            gameObjectInstanceGUID = Guid.NewGuid();
            gameObjectInstance = new UnityEngine.GameObject();

            containingType = typeManager.GetType(typeIdentifier);

            this.parentGameObject = parentGameObject;
            childGameObjects = new GameObjectRegistry();

            containedComponents = new ComponentRegistry();
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
