using System;
using UnityEngine;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public class NamespaceIdentifier : Identifier, INamespaceIdentifier
    {
        #region Properties
        public string NamespaceGUSID => GUSID;

        public string NamespaceName => namespaceName;
#nullable enable
        public INamespaceIdentifier? ParentNamespaceIdentifier => parentNamespaceIdentifier;
#nullable disable
        #endregion

        #region Fields
        [SerializeField] private readonly string namespaceName;
#nullable enable
        [SerializeField] private readonly INamespaceIdentifier? parentNamespaceIdentifier;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        protected NamespaceIdentifier(string namespaceName, NamespaceIdentifier? parentNamespaceIdentifier = null) : base(parentNamespaceIdentifier == null ? $"{namespaceName}" : $"{parentNamespaceIdentifier}.{namespaceName}")
        {
            this.namespaceName = namespaceName;
            this.parentNamespaceIdentifier = parentNamespaceIdentifier;
        }
#nullable disable
        #endregion

        #region Static Methods
#nullable enable
        public static NamespaceIdentifier Parse(string namespaceGUSID)
        {
            if (!TryParse(namespaceGUSID, out NamespaceIdentifier? namespaceIdentifier))
            {
                throw new ArgumentException($"'{namespaceGUSID}' is not a valid namespace GUSID!");
            }

            return namespaceIdentifier!;
        }

        public static bool TryParse(string namespaceGUSID, out NamespaceIdentifier? namespaceIdentifier)
        {
            namespaceIdentifier = null;

            if (string.IsNullOrEmpty(namespaceGUSID) || string.IsNullOrWhiteSpace(namespaceGUSID))
            {
                return false;
            }

            string[] parts = namespaceGUSID.Split(new char[] { '.' }, StringSplitOptions.RemoveEmptyEntries);

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
#nullable disable

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

            foreach (char character in namespaceName)
            {
                if (!char.IsLetterOrDigit(character) && character != '_')
                {
                    return false;
                }
            }

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

#nullable enable
        public static implicit operator NamespaceIdentifier?(string @namespace)
        {
            if (TryParse(@namespace, out NamespaceIdentifier? namespaceIdentifier))
            {
                return namespaceIdentifier;
            }
            else
            {
                throw new ArgumentException($"The string '{@namespace}' is not a valid namespace.");
            }
        }
#nullable disable
        #endregion
    }
}
