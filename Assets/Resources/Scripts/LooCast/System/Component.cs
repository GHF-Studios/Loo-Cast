using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Registration;

    public class Component
    {
        #region Properties
        public ComponentIdentifier Identifier
        {
            get
            {
                if (identifier == null)
                {
                    identifier = new ComponentIdentifier(ContainingType.Identifier, ContainingGameObject.Identifier, ComponentInstanceGUID);
                }
                return identifier.Value;
            }
        }

        public Guid ComponentInstanceGUID => componentInstanceGUID;
        public UnityEngine.Component ComponentInstance => componentInstance;

        public Type ContainingType => containingType;
        public GameObject ContainingGameObject => containingGameObject;
        #endregion

        #region Fields
#nullable enable 
        private ComponentIdentifier? identifier;
#nullable disable

        private Guid componentInstanceGUID;
        private UnityEngine.Component componentInstance;

        private Type containingType;
        private GameObject containingGameObject;
        #endregion

        #region Constructors
        public Component(Guid componentInstanceGUID, UnityEngine.Component componentInstance, Type containingType, GameObject containingGameObject)
        {
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
            return Identifier.Equals(otherComponent.Identifier);
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
