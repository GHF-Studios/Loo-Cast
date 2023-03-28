using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Registration;

    public class GameObject
    {
        #region Properties
        public GameObjectIdentifier Identifier
        {
            get
            {
                if (identifier == null)
                {
                    identifier = new GameObjectIdentifier(ContainingType.Identifier, InstanceGUID);
                }
                return identifier.Value;
            }
        }
        
        public Guid InstanceGUID => instanceGUID;
        public UnityEngine.Object Instance => instance;
        
        public Type ContainingType => containingType;
        
        public GameObject? ParentGameObject => parentGameObject;
        public GameObjectRegistry ChildGameObjects => childGameObjects;
        
        public ComponentRegistry ContainedComponents => containedComponents;
        #endregion

        #region Fields
        private GameObjectIdentifier? identifier;
        
        private Guid instanceGUID;
        private UnityEngine.Object instance;

        private Type containingType;

        private GameObject? parentGameObject;
        private GameObjectRegistry childGameObjects;

        private ComponentRegistry containedComponents;
        #endregion

        #region Constructors
        public GameObject(Guid instanceGUID, UnityEngine.Object instance, Type containingType)
        {
            this.instanceGUID = instanceGUID;
            this.instance = instance;
            
            this.containingType = containingType;

            parentGameObject = null;
            childGameObjects = new GameObjectRegistry();

            containedComponents = new ComponentRegistry();
        }

        public GameObject(Guid instanceGUID, UnityEngine.Object instance, GameObject parentGameObject)
        {
            this.instanceGUID = instanceGUID;
            this.instance = instance;

            containingType = parentGameObject.containingType;

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
            return Identifier.Equals(otherGameObject.Identifier);
        }

        public override int GetHashCode()
        {
            return Identifier.GetHashCode();
        }

        public override string ToString()
        {
            return Identifier.ToString();
        }
        #endregion
    }
}
