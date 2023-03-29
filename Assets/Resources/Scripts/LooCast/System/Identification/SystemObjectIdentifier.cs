using System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [Serializable]
    public struct SystemObjectIdentifier : IIdentifier
    {
        #region Properties
        public string GUSID
        {
            get
            {
                return $"{ContainingTypeIdentifier}[{SystemObjectInstanceGUID}]";
            }
        }
        public TypeIdentifier ContainingTypeIdentifier => containingTypeIdentifier;
        public Guid SystemObjectInstanceGUID => systemObjectInstanceGUID;
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier containingTypeIdentifier;
        [SerializeField] private Guid systemObjectInstanceGUID;
        #endregion

        #region Constructors
        public SystemObjectIdentifier(TypeIdentifier containingTypeIdentifier, Guid systemObjectInstanceGUID)
        {
            this.containingTypeIdentifier = containingTypeIdentifier;
            this.systemObjectInstanceGUID = systemObjectInstanceGUID;
        }
        #endregion

        #region Static Methods
        public static bool TryParse(string gusid, out SystemObjectIdentifier? systemObjectIdentifier)
        {
            systemObjectIdentifier = null;

            string[] parts = gusid.Split(new char[] { '[', ']' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string typeIdentifierString = parts[0];
            string systemObjectInstanceGUIDString = parts[1];

            if (!TypeIdentifier.TryParse(typeIdentifierString, out TypeIdentifier? containingTypeIdentifier))
            {
                return false;
            }

            if (!Guid.TryParse(systemObjectInstanceGUIDString, out Guid systemObjectInstanceGUID))
            {
                return false;
            }

            systemObjectIdentifier = new SystemObjectIdentifier(containingTypeIdentifier.Value, systemObjectInstanceGUID);
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
            if (obj is SystemObjectIdentifier)
            {
                return Equals((SystemObjectIdentifier)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(SystemObjectIdentifier otherSystemObjectIdentifier)
        {
            return otherSystemObjectIdentifier.GUSID.Equals(this.GUSID);
        }

        public override int GetHashCode()
        {
            return GUSID.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(SystemObjectIdentifier systemObjectIdentifier1, SystemObjectIdentifier systemObjectIdentifier2)
        {
            return systemObjectIdentifier1.Equals(systemObjectIdentifier2);
        }

        public static bool operator !=(SystemObjectIdentifier systemObjectIdentifier1, SystemObjectIdentifier systemObjectIdentifier2)
        {
            return !systemObjectIdentifier1.Equals(systemObjectIdentifier2);
        }

        public static implicit operator string(SystemObjectIdentifier systemObjectIdentifier)
        {
            return systemObjectIdentifier.GUSID;
        }

        public static implicit operator SystemObjectIdentifier?(string gusid)
        {
            if (TryParse(gusid, out SystemObjectIdentifier? systemObjectIdentifier))
            {
                return systemObjectIdentifier.Value;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid GUSID.");
            }
        }
        #endregion
    }
}
