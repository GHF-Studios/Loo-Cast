using System;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    public class Component : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => componentIdentifier;
        public ComponentIdentifier ComponentIdentifier => componentIdentifier;

        public Guid ComponentInstanceGUID => componentInstanceGUID;
        public UnityEngine.Component ComponentInstance => componentInstance;

        public Type ContainingType => containingType;
        public GameObject ContainingGameObject => containingGameObject;
        #endregion

        #region Fields
#nullable enable 
        private ComponentIdentifier? componentIdentifier;
#nullable disable

        private Guid componentInstanceGUID;
        private UnityEngine.Component componentInstance;

        private Type containingType;
        private GameObject containingGameObject;
        #endregion

        #region Constructors
        public Component(Guid componentInstanceGUID, UnityEngine.Component componentInstance, Type containingType, GameObject containingGameObject)
        {
            componentIdentifier = new ComponentIdentifier(containingGameObject.GameObjectIdentifier, containingType.TypeIdentifier, componentInstanceGUID);
            
            this.componentInstanceGUID = componentInstanceGUID;
            this.componentInstance = componentInstance;

            this.containingType = containingType;
            this.containingGameObject = containingGameObject;
        }
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
