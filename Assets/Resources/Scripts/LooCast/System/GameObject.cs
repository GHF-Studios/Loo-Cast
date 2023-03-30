using System;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    public class GameObject : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => gameObjectIdentifier;
        public GameObjectIdentifier GameObjectIdentifier => gameObjectIdentifier;

        public Guid GameObjectInstanceGUID => gameObjectInstanceGUID;
        public UnityEngine.Object GameObjectInstance => gameObjectInstance;
        
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
        private UnityEngine.Object gameObjectInstance;

        private Type containingType;

#nullable enable
        private GameObject? parentGameObject;
#nullable disable
        private GameObjectRegistry childGameObjects;

        private ComponentRegistry containedComponents;
        #endregion

        #region Constructors
        public GameObject(Guid gameObjectInstanceGUID, UnityEngine.Object gameObjectInstance, Type containingType, GameObject parentGameObject = null)
        {
            gameObjectIdentifier = new GameObjectIdentifier(ContainingType.TypeIdentifier, GameObjectInstanceGUID);
            
            this.gameObjectInstanceGUID = gameObjectInstanceGUID;
            this.gameObjectInstance = gameObjectInstance;
            
            this.containingType = containingType;

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
