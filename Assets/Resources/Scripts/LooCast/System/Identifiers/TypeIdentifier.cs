using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public class TypeIdentifier : Identifier
    {
        #region Properties
        public NamespaceIdentifier TypeNamespaceIdentifier => typeNamespaceIdentifier;
        public string FullTypeName => fullTypeName;
        public global::System.Type CSSystemType => cssystemType;
#nullable enable
        public TypeIdentifier[]? GenericTypeArgumentIdentifiers => genericTypeArgumentIdentifiers;
#nullable disable
        #endregion

        #region Fields
        [SerializeField] private readonly NamespaceIdentifier typeNamespaceIdentifier;
        [SerializeField] private readonly string fullTypeName;
        private readonly global::System.Type cssystemType;
#nullable enable
        [SerializeField] private readonly TypeIdentifier[]? genericTypeArgumentIdentifiers;
#nullable disable
        #endregion
         
        #region Constructors
#nullable enable
        private TypeIdentifier(NamespaceIdentifier typeNamespaceIdentifier, string fullTypeName, global::System.Type cssystemType, TypeIdentifier[]? genericTypeArgumentIdentifiers) : base($"{typeNamespaceIdentifier}:{fullTypeName}")
        {
            this.typeNamespaceIdentifier = typeNamespaceIdentifier;
            this.fullTypeName = fullTypeName;
            this.cssystemType = cssystemType;
            this.genericTypeArgumentIdentifiers = genericTypeArgumentIdentifiers;
        }
#nullable disable
        #endregion

        #region Static Methods
#nullable enable
        public static TypeIdentifier Parse(global::System.Type cssystemType)
        {
            NamespaceIdentifier typeNamespaceIdentifier = NamespaceIdentifier.Parse(cssystemType.Namespace);

            string fullTypeName = cssystemType.Name.Replace("+", ".");
            string[] typeParts = fullTypeName.Split('`');
            fullTypeName = typeParts[0];

            TypeIdentifier[]? genericTypeArguments = null;

            if (typeParts.Length > 1)
            {
                fullTypeName += "<";
                int numTypeArgs = int.Parse(typeParts[1]);

                global::System.Type[] genericArgumentTypes = cssystemType.GetGenericArguments();
                genericTypeArguments = new TypeIdentifier[genericArgumentTypes.Length];

                for (int i = 0; i < numTypeArgs; i++)
                {
                    if (i > 0)
                    {
                        fullTypeName += ", ";
                    }

                    TypeIdentifier genericArgumentTypeIdentifier = Parse(genericArgumentTypes[i]);

                    fullTypeName += genericArgumentTypeIdentifier.ToString();
                    genericTypeArguments[i] = genericArgumentTypeIdentifier;
                }

                fullTypeName += ">";
            }

            return new TypeIdentifier(typeNamespaceIdentifier, fullTypeName, cssystemType, genericTypeArguments);
        }

        public static bool TryParse(string gusid, out TypeIdentifier? typeIdentifier)
        {
            typeIdentifier = null;

            string[] parts = gusid.Split(new char[] { ':' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string typeNamespaceIdentifierString = parts[0];
            string fullTypeName = parts[1];

            if (!NamespaceIdentifier.TryParse(typeNamespaceIdentifierString, out NamespaceIdentifier? typeNamespaceIdentifier))
            {
                return false;
            }

            if (!IsValidFullTypeName(fullTypeName))
            {
                return false;
            }

            global::System.Type? cssystemType = null;
            TypeIdentifier[]? genericTypeArguments = null;

            int genericTypeStartIndex = fullTypeName.IndexOf("<");
            if (genericTypeStartIndex >= 0)
            {
                int genericTypeEndIndex = fullTypeName.LastIndexOf(">");
                if (genericTypeEndIndex > genericTypeStartIndex)
                {
                    string genericTypeArgsString = fullTypeName.Substring(genericTypeStartIndex + 1, genericTypeEndIndex - genericTypeStartIndex - 1);
                    string[] genericTypeArgumentStrings = genericTypeArgsString.Split(new[] { ',' }, StringSplitOptions.RemoveEmptyEntries);
                    genericTypeArguments = new TypeIdentifier[genericTypeArgumentStrings.Length];
                    for (int i = 0; i < genericTypeArgumentStrings.Length; i++)
                    {
                        string genericTypeArgumentString = genericTypeArgumentStrings[i].Trim();
                        if (!TryParse(genericTypeArgumentString, out TypeIdentifier? genericTypeArgumentType))
                        {
                            return false;
                        }
                        genericTypeArguments[i] = genericTypeArgumentType!;
                    }

                    fullTypeName = fullTypeName.Substring(0, genericTypeStartIndex);
                }
                else
                {
                    return false;
                }
            }

            cssystemType = global::System.Type.GetType($"{typeNamespaceIdentifier}.{fullTypeName.Replace(".", "+")}");
            if (cssystemType == null)
            {
                return false;
            }

            typeIdentifier = new TypeIdentifier(typeNamespaceIdentifier!, fullTypeName, cssystemType, genericTypeArguments);
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

            foreach (char character in typeName)
            {
                if (!char.IsLetterOrDigit(character) && character != '_' && character != '<' && character != '>')
                {
                    return false;
                }
            }

            return true;
        }

        private static bool IsValidFullTypeName(string fullTypeName)
        {
            if (string.IsNullOrEmpty(fullTypeName) || string.IsNullOrWhiteSpace(fullTypeName))
            {
                return false;
            }

            string[] parts = fullTypeName.Split(new char[] { '.' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length == 0)
            {
                return false;
            }

            foreach (string part in parts)
            {
                if (!IsValidTypeName(part))
                {
                    return false;
                }
            }

            int genericStartIndex = fullTypeName.IndexOf('<');
            int genericEndIndex = fullTypeName.LastIndexOf('>');

            if (genericStartIndex > -1 && genericEndIndex > -1 && genericStartIndex < genericEndIndex)
            {
                string genericTypeArguments = fullTypeName.Substring(genericStartIndex + 1, genericEndIndex - genericStartIndex - 1);

                foreach (string genericTypeArgument in genericTypeArguments.Split(','))
                {
                    if (!IsValidFullTypeName(genericTypeArgument.Trim()))
                    {
                        return false;
                    }
                }
            }

            int nestedStartIndex = fullTypeName.IndexOf('+');

            while (nestedStartIndex > -1)
            {
                int nestedEndIndex = fullTypeName.IndexOf(',', nestedStartIndex);

                if (nestedEndIndex == -1)
                {
                    nestedEndIndex = fullTypeName.Length;
                }

                string nestedTypeName = fullTypeName.Substring(nestedStartIndex + 1, nestedEndIndex - nestedStartIndex - 1);

                if (!IsValidFullTypeName(nestedTypeName))
                {
                    return false;
                }

                nestedStartIndex = fullTypeName.IndexOf('+', nestedEndIndex);
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
                throw new ArgumentException($"The string '{gusid}' could not be parsed!");
            }
        }
        
        public static implicit operator TypeIdentifier?(global::System.Type cssystemType)
        {
            return Parse(cssystemType);
        }
#nullable disable
        #endregion
    }
}
