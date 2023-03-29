using System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [Serializable]
    public struct TypeIdentifier : IIdentifier
    {
        #region Properties
        public string GUSID
        {
            get
            {
                return $"{ContainingNamespaceIdentifier}:{TypeName}";
            }
        }
        public NamespaceIdentifier ContainingNamespaceIdentifier => containingNamespaceIdentifier;
        public string TypeName => typeName;
        #endregion

        #region Fields
        [SerializeField] private NamespaceIdentifier containingNamespaceIdentifier;
        [SerializeField] private string typeName;
        #endregion
        
        #region Constructors
        public TypeIdentifier(NamespaceIdentifier containingNamespaceIdentifier, string typeName)
        {
            if (!IsValidTypeName(typeName))
            {
                throw new ArgumentException($"Invalid Type Name: '{typeName}'");
            }

            this.containingNamespaceIdentifier = containingNamespaceIdentifier;
            this.typeName = typeName;
        }
        #endregion

        #region Static Methods
        public static bool TryParse(string gusid, out TypeIdentifier? typeIdentifier)
        {
            typeIdentifier = null;

            string[] parts = gusid.Split(new char[] { ':' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string containingNamespaceIdentifierString = parts[0];
            string typeName = parts[1];

            if (!NamespaceIdentifier.TryParse(containingNamespaceIdentifierString, out NamespaceIdentifier? containingNamespaceIdentifier))
            {
                return false;
            }

            if (!IsValidTypeName(typeName))
            {
                return false;
            }

            typeIdentifier = new TypeIdentifier(containingNamespaceIdentifier.Value, typeName);
            return true;
        }

        private static bool IsValidTypeName(string typeName)
        {
            if (string.IsNullOrEmpty(typeName) || string.IsNullOrWhiteSpace(typeName))
            {
                return false;
            }

            if (!char.IsLetter(typeName[0]))
            {
                return false;
            }

            foreach (char c in typeName)
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
        public override string ToString()
        {
            return GUSID;
        }
        
        public override bool Equals(object obj)
        {
            if (obj is TypeIdentifier)
            {
                return Equals((TypeIdentifier)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(TypeIdentifier otherTypeIdentifier)
        {
            return otherTypeIdentifier.GUSID.Equals(this.GUSID);
        }

        public override int GetHashCode()
        {
            return GUSID.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(TypeIdentifier typeIdentifier1, TypeIdentifier typeIdentifier2)
        {
            return typeIdentifier1.Equals(typeIdentifier2);
        }

        public static bool operator !=(TypeIdentifier typeIdentifier1, TypeIdentifier typeIdentifier2)
        {
            return !typeIdentifier1.Equals(typeIdentifier2);
        }

        public static implicit operator string(TypeIdentifier typeIdentifier)
        {
            return typeIdentifier.GUSID;
        }

        public static implicit operator TypeIdentifier(string gusid)
        {
            if (TryParse(gusid, out TypeIdentifier? typeIdentifier))
            {
                return typeIdentifier.Value;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid GUSID.");
            }
        }
        #endregion
    }
}
