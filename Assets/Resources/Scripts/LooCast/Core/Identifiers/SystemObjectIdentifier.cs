using System;
using UnityEngine;

namespace LooCast.Core.Identifiers
{
    [Serializable]
    public class SystemObjectIdentifier : InstanceIdentifier, ISystemObjectIdentifier
    {
        #region Properties
        public TypeIdentifier SystemObjectTypeIdentifier => systemObjectTypeIdentifier;
        public Guid SystemObjectInstanceGUID => systemObjectInstanceGUID;
        #endregion

        #region Fields
        [SerializeField] private readonly TypeIdentifier systemObjectTypeIdentifier;
        [SerializeField] private readonly Guid systemObjectInstanceGUID;
        #endregion

        #region Constructors
        public SystemObjectIdentifier(TypeIdentifier systemObjectTypeIdentifier, Guid systemObjectInstanceGUID, string gusid = null) : base(gusid == null ? $"{systemObjectTypeIdentifier}[{systemObjectInstanceGUID}]" : gusid)
        {
            this.systemObjectTypeIdentifier = systemObjectTypeIdentifier;
            this.systemObjectInstanceGUID = systemObjectInstanceGUID;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusid, out SystemObjectIdentifier? systemObjectIdentifier)
        {
            systemObjectIdentifier = null;

            string[] parts = gusid.Split(new char[] { '[', ']' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string systemObjectTypeIdentifierString = parts[0];
            string systemObjectInstanceGUIDString = parts[1];

            if (!TypeIdentifier.TryParse(systemObjectTypeIdentifierString, out TypeIdentifier? systemObjectTypeIdentifier))
            {
                return false;
            }

            if (!Guid.TryParse(systemObjectInstanceGUIDString, out Guid systemObjectInstanceGUID))
            {
                return false;
            }

            systemObjectIdentifier = new SystemObjectIdentifier(systemObjectTypeIdentifier, systemObjectInstanceGUID);
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

#nullable enable
        public static implicit operator SystemObjectIdentifier?(string gusid)
        {
            if (TryParse(gusid, out SystemObjectIdentifier? systemObjectIdentifier))
            {
                return systemObjectIdentifier;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid SystemObject GUSID.");
            }
        }
#nullable disable
        #endregion
    }
}
