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
                return $"{TypeIdentifier}[{InstanceGUID}]";
            }
        }
        public TypeIdentifier TypeIdentifier => typeIdentifier;
        public Guid InstanceGUID => instanceGUID;
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier typeIdentifier;
        [SerializeField] private Guid instanceGUID;
        #endregion

        #region Constructors
        public SystemObjectIdentifier(TypeIdentifier typeIdentifier, Guid instanceGUID = new Guid())
        {
            this.typeIdentifier = typeIdentifier;
            this.instanceGUID = instanceGUID;
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
            string instanceGUIDString = parts[1];

            if (!TypeIdentifier.TryParse(typeIdentifierString, out TypeIdentifier? typeIdentifier))
            {
                return false;
            }

            if (!Guid.TryParse(instanceGUIDString, out Guid instanceGUID))
            {
                return false;
            }

            systemObjectIdentifier = new SystemObjectIdentifier(typeIdentifier.Value, instanceGUID);
            return true;
        }
        #endregion

        #region Overrides
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

        public override string ToString()
        {
            return GUSID;
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
