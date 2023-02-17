using CSSystem = System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [CSSystem.Serializable]
    public class TypeIdentifier : ITypeIdentifier
    {
        #region Properties
        public string ParentNamespaceID => parentNamespaceID;
        public string ParentTypeID => parentTypeID;
        public CSSystem.Type CSSystemType => cssystemType;
        public string TypeID => parentTypeID == null ? $"{parentNamespaceID}:{cssystemType.FullName}" : $"{parentTypeID}.{cssystemType.FullName}";
        public string ID => TypeID;
        #endregion

        #region Fields
        [SerializeField] protected string parentNamespaceID;
        [SerializeField] protected string parentTypeID;
        [SerializeField] protected CSSystem.Type cssystemType; // TODO: Use serializable Type instead of CSSystem.Type
        #endregion

        #region Constructors
        public TypeIdentifier(NamespaceIdentifier parentNamespace, CSSystem.Type systemType)
        {
            parentNamespaceID = parentNamespace.NamespaceID;
            parentTypeID = null;
            this.cssystemType = systemType;
        }

        public TypeIdentifier(TypeIdentifier parentType, CSSystem.Type systemType)
        {
            parentNamespaceID = parentType.parentNamespaceID;
            parentTypeID = parentType.TypeID;
            this.cssystemType = systemType;
        }

        public TypeIdentifier(string typeID)
        {
            string[] typeIDParts = typeID.Split(':');
            string[] typeIDParts2 = typeIDParts[1].Split('.');
            parentNamespaceID = typeIDParts[0];
            cssystemType = CSSystem.Type.GetType($"{typeIDParts[0]}.{typeIDParts[1]}");
            parentTypeID = typeIDParts2.Length > 1 ? $"{typeIDParts[0]}:{typeID.Substring(0, typeID.Length - typeIDParts2[typeIDParts2.Length - 1].Length - 1)}" : null;
        }
        #endregion

        #region Operators
        public static implicit operator TypeIdentifier(string typeID)
        {
            return new TypeIdentifier(typeID);
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (!(obj is TypeIdentifier))
            {
                return false;
            }
            TypeIdentifier otherTypeIdentifier = (TypeIdentifier)obj;
            return this.Equals(otherTypeIdentifier);
        }

        public bool Equals(TypeIdentifier other)
        {
            return TypeID == other.TypeID;
        }

        public override int GetHashCode()
        {
            return TypeID.GetHashCode();
        }

        public static bool operator ==(TypeIdentifier left, TypeIdentifier right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(TypeIdentifier left, TypeIdentifier right)
        {
            return !(left == right);
        }

        public override string ToString()
        {
            return TypeID;
        }
        #endregion
    }
}
