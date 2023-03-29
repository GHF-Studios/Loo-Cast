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
        public UnityEngine.Object ComponentInstance => componentInstance;

        public Type ContainingType => containingType;
        public GameObject ContainingGameObject => containingGameObject;
        #endregion

        #region Fields
#nullable enable 
        private ComponentIdentifier? identifier;
#nullable disable

        private Guid componentInstanceGUID;
        private UnityEngine.Object componentInstance;

        private Type containingType;
        private GameObject? containingGameObject;
        #endregion
    }
}
