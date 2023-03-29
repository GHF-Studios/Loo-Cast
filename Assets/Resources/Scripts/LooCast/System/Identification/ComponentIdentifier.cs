using System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [Serializable]
    public struct ComponentIdentifier : IIdentifier
    {
        #region Properties
        public string GUSID
        {
            get
            {
                return $"{ContainingTypeIdentifier}{{{ComponentInstanceGUID}}}";
            }
        }
        public TypeIdentifier ContainingTypeIdentifier => containingTypeIdentifier;
        public GameObjectIdentifier ContainingGameObject => containingGameObject;
        public Guid ComponentInstanceGUID => componentInstanceGUID;
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier containingTypeIdentifier;
        [SerializeField] private GameObjectIdentifier containingGameObject;
        [SerializeField] private Guid componentInstanceGUID;
        #endregion

        #region Constructors
        public ComponentIdentifier(TypeIdentifier containingTypeIdentifier, GameObjectIdentifier containingGameObject, Guid componentInstanceGUID)
        {
            this.containingTypeIdentifier = containingTypeIdentifier;
            this.containingGameObject = containingGameObject;
            this.componentInstanceGUID = componentInstanceGUID;
        }
        #endregion

        #region Static Methods
        public static bool TryParse(string gusid, out ComponentIdentifier? componentIdentifier)
        {
            componentIdentifier = null;

            string[] parts = gusid.Split(new char[] { '{', '}' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string containingGameObjectIdentifierString = parts[0];
            string componentInstanceGUIDString = parts[1];

            if (!GameObjectIdentifier.TryParse(containingGameObjectIdentifierString, out GameObjectIdentifier? containingGameObjectIdentifier))
            {
                return false;
            }

            if (!Guid.TryParse(componentInstanceGUIDString, out Guid componentInstanceGUID))
            {
                return false;
            }

            componentIdentifier = new ComponentIdentifier(containingGameObjectIdentifier.Value.ContainingTypeIdentifier, containingGameObjectIdentifier.Value, componentInstanceGUID);
            return true;
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return GUSID;
        }

        public override bool Equals(object obj)
        {
            if (obj is ComponentIdentifier)
            {
                return Equals((ComponentIdentifier)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(ComponentIdentifier otherComponentIdentifier)
        {
            return otherComponentIdentifier.GUSID.Equals(this.GUSID);
        }

        public override int GetHashCode()
        {
            return GUSID.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(ComponentIdentifier componentIdentifier1, ComponentIdentifier componentIdentifier2)
        {
            return componentIdentifier1.Equals(componentIdentifier2);
        }

        public static bool operator !=(ComponentIdentifier componentIdentifier1, ComponentIdentifier componentIdentifier2)
        {
            return !componentIdentifier1.Equals(componentIdentifier2);
        }

        public static implicit operator string(ComponentIdentifier componentIdentifier)
        {
            return componentIdentifier.GUSID;
        }

        public static implicit operator ComponentIdentifier(string gusid)
        {
            if (TryParse(gusid, out ComponentIdentifier? componentIdentifier))
            {
                return componentIdentifier.Value;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid GUSID.");
            }
        }
        #endregion
    }
}
