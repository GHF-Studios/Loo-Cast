using System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [Serializable]
    public struct NamespaceIdentifier : IIdentifier
    {
        #region Properties
        public string GUSID
        {
            get
            {
                return ParentNamespaceIdentifier == null ? $"{NamespaceName}" : $"{ParentNamespaceIdentifier}.{NamespaceName}";
            }
        }
        public NamespaceIdentifier? ParentNamespaceIdentifier
        {
            get
            {
                return !NamespaceIdentifier.TryParse(parentNamespaceGUSID, out NamespaceIdentifier namespaceIdentifier) ? namespaceIdentifier : null;
            }
        }
        public string NamespaceName => namespaceName;
        #endregion

        #region Fields
        [SerializeField] private string parentNamespaceGUSID;
        [SerializeField] private string namespaceName;
        #endregion

        #region Constructors
        public NamespaceIdentifier(string namespaceName, NamespaceIdentifier? parentNamespaceIdentifier = null)
        {
            this.namespaceName = namespaceName;
            parentNamespaceGUSID = parentNamespaceIdentifier == null ? null : parentNamespaceIdentifier.Value.GUSID;
        }
        #endregion

        #region Static Methods
        public static bool TryParse(string gusid, out NamespaceIdentifier? namespaceIdentifier)
        {
            namespaceIdentifier = null;

            if (string.IsNullOrEmpty(gusid) || string.IsNullOrWhiteSpace(gusid))
            {
                return false;
            }

            string[] parts = gusid.Split(new char[] { '.' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length == 0)
            {
                return false;
            }

            NamespaceIdentifier? currentNamespaceIdentifier = null;

            for (int i = 0; i < parts.Length; i++)
            {
                if (!IsValidNamespaceName(parts[i]))
                {
                    return false;
                }

                currentNamespaceIdentifier = new NamespaceIdentifier(parts[i], currentNamespaceIdentifier);

                if (i == parts.Length - 1)
                {
                    namespaceIdentifier = currentNamespaceIdentifier;
                }
            }

            return namespaceIdentifier != null;
        }

        private static bool IsValidNamespaceName(string namespaceName)
        {
            if (string.IsNullOrEmpty(namespaceName) || string.IsNullOrWhiteSpace(namespaceName))
            {
                return false;
            }

            if (!char.IsLetter(namespaceName[0]))
            {
                return false;
            }

            foreach (char c in namespaceName)
            {
                if (!char.IsLetterOrDigit(c) && c != '_')
                {
                    return false;
                }
            }

            return true;
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is NamespaceIdentifier)
            {
                return Equals((NamespaceIdentifier)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(NamespaceIdentifier otherNamespaceIdentifier)
        {
            return otherNamespaceIdentifier.GUSID.Equals(this.GUSID);
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
        public static bool operator ==(NamespaceIdentifier namespaceIdentifier1, NamespaceIdentifier namespaceIdentifier2)
        {
            return namespaceIdentifier1.Equals(namespaceIdentifier2);
        }

        public static bool operator !=(NamespaceIdentifier namespaceIdentifier1, NamespaceIdentifier namespaceIdentifier2)
        {
            return !namespaceIdentifier1.Equals(namespaceIdentifier2);
        }

        public static implicit operator string(NamespaceIdentifier namespaceIdentifier)
        {
            return namespaceIdentifier.GUSID;
        }

        public static implicit operator NamespaceIdentifier(string gusid)
        {
            if (TryParse(gusid, out NamespaceIdentifier? namespaceIdentifier))
            {
                return namespaceIdentifier.Value;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid GUSID.");
            }
        }
        #endregion
    }
}
