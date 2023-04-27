using System;
using UnityEngine;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public class ComponentIdentifier : InstanceIdentifier, IComponentIdentifier
    {
        #region Properties
        public TypeIdentifier ComponentTypeIdentifier => componentTypeIdentifier;
        public Guid ComponentInstanceGUID => componentInstanceGUID;
        public GameObjectIdentifier ContainingGameObjectIdentifier => containingGameObjectIdentifier;
        #endregion

        #region Fields
        [SerializeField] private readonly TypeIdentifier componentTypeIdentifier;
        [SerializeField] private readonly Guid componentInstanceGUID;
        [SerializeField] private readonly GameObjectIdentifier containingGameObjectIdentifier;
        #endregion

        #region Constructors
        public ComponentIdentifier(TypeIdentifier componentTypeIdentifier, Guid componentInstanceGUID, GameObjectIdentifier containingGameObjectIdentifier, string gusid = null) : base(gusid == null ? $"{containingGameObjectIdentifier}{{{componentTypeIdentifier}({componentInstanceGUID})}}" : gusid)
        {
            this.containingGameObjectIdentifier = containingGameObjectIdentifier;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusid, out ComponentIdentifier? componentIdentifier)
        {
            componentIdentifier = null;

            string[] parts = gusid.Split(new char[] { '{', '}' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string componentGameObjectIdentifierString = parts[0];
            string componentIdentifierString = parts[1];

#pragma warning disable CS8600
            if (!GameObjectIdentifier.TryParse(componentGameObjectIdentifierString, out GameObjectIdentifier componentGameObjectIdentifier))
            {
                return false;
            }

            if (!GameObjectIdentifier.TryParse(componentIdentifierString, out GameObjectIdentifier gameObjectIdentifier))
            {
                return false;
            }
#pragma warning restore CS8600

#pragma warning disable CS8602
            componentIdentifier = new ComponentIdentifier(componentGameObjectIdentifier, gameObjectIdentifier.GameObjectTypeIdentifier, gameObjectIdentifier.GameObjectInstanceGUID);
#pragma warning restore CS8602
            return true;
        }
#nullable disable
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

#nullable enable
        public static implicit operator ComponentIdentifier?(string gusid)
        {
            if (TryParse(gusid, out ComponentIdentifier? componentIdentifier))
            {
                return componentIdentifier;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid Component GUSID.");
            }
        }
#nullable disable
        #endregion
    }
}
