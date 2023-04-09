using System;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;

    public class Component : IHierarchyElement
    {
        #region Properties
        public Identifier Identifier => componentIdentifier;
        public ComponentIdentifier ComponentIdentifier => componentIdentifier;

        public Guid ComponentInstanceGUID => componentInstanceGUID;
        public UnityEngine.Component ComponentInstance => componentInstance;

        public Type ComponentType => componentType;
        public GameObject ContainingGameObject => containingGameObject;
        #endregion

        #region Fields
        private ComponentIdentifier componentIdentifier;

        private Guid componentInstanceGUID;
        private UnityEngine.Component componentInstance;

        private Type componentType;
        private GameObject containingGameObject;
        #endregion

        #region Constructors
#nullable enable 
        public Component(Type componentType, GameObject containingGameObject)
        {
            this.componentType = componentType;
            this.containingGameObject = containingGameObject;
            
            componentIdentifier = new ComponentIdentifier(containingGameObject.GameObjectIdentifier, componentType.TypeIdentifier, Guid.NewGuid());
            componentInstanceGUID = componentIdentifier.ComponentInstanceGUID;
            componentInstance = containingGameObject.GameObjectInstance.AddComponent<ExtendedMonoBehaviour>();

            containingGameObject.ContainedComponents.Add(this);
        }
#nullable disable
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is Component otherComponent)
            {
                return Equals(otherComponent);
            }
            return false;
        }

        public bool Equals(Component otherComponent)
        {
            return ComponentIdentifier.Equals(otherComponent.ComponentIdentifier);
        }

        public override int GetHashCode()
        {
            return ComponentIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return ComponentIdentifier.ToString();
        }
        #endregion
    }
}
