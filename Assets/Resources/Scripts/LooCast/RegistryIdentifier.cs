using System;
using UnityEngine;

namespace LooCast
{
    [Serializable]
    public record RegistryIdentifier : IGenericIdentifier<Registry<IIdentifier, IIdentifiable>>
    {
        #region Properties
        public string KeyTypeID => keyTypeID;
        public string ValueTypeID => valuetypeID;

        public string RegistryID => $"{KeyTypeID}_{ValueTypeID}";
        public string ID => RegistryID;
        #endregion

        #region Fields
        [SerializeField] private string keyTypeID;
        [SerializeField] private string valuetypeID;
        #endregion

        #region Constructors
        internal RegistryIdentifier(string keyTypeID, string valuetypeID)
        {
            this.keyTypeID = keyTypeID;
            this.valuetypeID = valuetypeID;
        }
        
        internal RegistryIdentifier(string registryID)
        {
            string[] registryIDParts = registryID.Split('_');
            if (registryIDParts.Length != 2)
            {
                throw new Exception($"[RegistryIdentifier] Invalid registry ID '{registryID}'!");
            }
            keyTypeID = registryIDParts[0];
            valuetypeID = registryIDParts[1];
        }
        #endregion

        #region Operators
        public static implicit operator RegistryIdentifier(string registryID)
        {
            return new RegistryIdentifier(registryID);
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (!(obj is RegistryIdentifier))
            {
                return false;
            }
            RegistryIdentifier otherRegistryIdentifier = (RegistryIdentifier)obj;
            return this.Equals(otherRegistryIdentifier);
        }

        public bool Equals(RegistryIdentifier other)
        {
            return RegistryID == other.RegistryID;
        }

        public override int GetHashCode()
        {
            return RegistryID.GetHashCode();
        }

        public static bool operator ==(RegistryIdentifier left, RegistryIdentifier right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(RegistryIdentifier left, RegistryIdentifier right)
        {
            return !(left == right);
        }

        public override string ToString()
        {
            return RegistryID;
        }
        #endregion
    }
}
