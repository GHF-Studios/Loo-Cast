using System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [Serializable]
    public class NamespaceIdentifier : INamespaceIdentifier
    {
        #region Properties
        public string ParentNamespaceID => parentNamespaceID;
        public string Name => name;
        public string NamespaceID => parentNamespaceID == null ? name : $"{parentNamespaceID}.{name}";
        public string ID => NamespaceID;
        #endregion

        #region Fields
        [SerializeField] private string parentNamespaceID;
        [SerializeField] private string name;
        #endregion

        #region Constructors
        internal NamespaceIdentifier(string name, string parentNamespaceID)
        {
            this.name = name;
            this.parentNamespaceID = parentNamespaceID;
        }

        internal NamespaceIdentifier(string namespaceID)
        {
            string[] namespaceIDParts = namespaceID.Split('.');
            name = namespaceIDParts[namespaceIDParts.Length - 1];
            parentNamespaceID = namespaceIDParts.Length > 1 ? namespaceID.Substring(0, namespaceID.Length - name.Length - 1) : null;
        }
        #endregion

        #region Operators
        public static implicit operator NamespaceIdentifier(string namespaceID)
        {
            return new NamespaceIdentifier(namespaceID);
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (!(obj is NamespaceIdentifier))
            {
                return false;
            }
            NamespaceIdentifier otherNamespaceIdentifier = (NamespaceIdentifier)obj;
            return this.Equals(otherNamespaceIdentifier);
        }

        public bool Equals(NamespaceIdentifier other)
        {
            return NamespaceID == other.NamespaceID;
        }

        public override int GetHashCode()
        {
            return NamespaceID.GetHashCode();
        }

        public static bool operator ==(NamespaceIdentifier left, NamespaceIdentifier right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(NamespaceIdentifier left, NamespaceIdentifier right)
        {
            return !(left == right);
        }

        public override string ToString()
        {
            return NamespaceID;
        }
        #endregion
    }
}
