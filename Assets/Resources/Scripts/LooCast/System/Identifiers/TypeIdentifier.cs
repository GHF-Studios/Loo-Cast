using System;
using UnityEngine;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public class TypeIdentifier : Identifier
    {
        #region Properties
        public NamespaceIdentifier TypeNamespaceIdentifier => typeNamespaceIdentifier;
        public string TypeName => typeName;
        #endregion

        #region Fields
        [SerializeField] private readonly NamespaceIdentifier typeNamespaceIdentifier;
        [SerializeField] private readonly string typeName;
        #endregion
        
        #region Constructors
        public TypeIdentifier(NamespaceIdentifier typeNamespaceIdentifier, string typeName, string gusid = null) : base(gusid == null ? $"{typeNamespaceIdentifier}:{typeName}" : gusid)
        {
            if (!IsValidTypeName(typeName))
            {
                throw new ArgumentException($"Invalid Type Name: '{typeName}'");
            }

            this.typeNamespaceIdentifier = typeNamespaceIdentifier;
            this.typeName = typeName;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusid, out TypeIdentifier? typeIdentifier)
        {
            typeIdentifier = null;

            string[] parts = gusid.Split(new char[] { ':' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string typeNamespaceIdentifierString = parts[0];
            string typeName = parts[1];

            if (!NamespaceIdentifier.TryParse(typeNamespaceIdentifierString, out NamespaceIdentifier? typeNamespaceIdentifier))
            {
                return false;
            }

            if (!IsValidTypeName(typeName))
            {
                return false;
            }

            typeIdentifier = new TypeIdentifier(typeNamespaceIdentifier, typeName);
            return true;
        }
#nullable disable

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

#nullable enable
        public static implicit operator TypeIdentifier?(string gusid)
        {
            if (TryParse(gusid, out TypeIdentifier? typeIdentifier))
            {
                return typeIdentifier;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid Type GUSID.");
            }
        }
#nullable disable
        #endregion
    }
}
