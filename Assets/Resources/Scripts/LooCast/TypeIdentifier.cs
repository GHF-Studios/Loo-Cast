using System;
using UnityEngine;

namespace LooCast
{
    [Serializable]
    public struct TypeIdentifier : IIdentifier
    {
        #region Properties
        public string ParentNamespaceID => parentNamespaceID;
        public string ParentTypeID => parentTypeID;
        public System.Type SystemType => systemType;
        
        public string TypeID => parentTypeID == null ? $"{parentNamespaceID}:{systemType.FullName}" : $"{parentTypeID}.{systemType.FullName}";
        public string ID => TypeID;
        #endregion

        #region Fields
        [SerializeField] private string parentNamespaceID;
        [SerializeField] private string parentTypeID;
        [SerializeField] private System.Type systemType; // TODO: Use serializable Type instead of System.Type
        #endregion

        #region Constructors
        internal TypeIdentifier(NamespaceIdentifier parentNamespace, System.Type systemType)
        {
            parentNamespaceID = parentNamespace.NamespaceID;
            parentTypeID = null;
            this.systemType = systemType;
        }

        internal TypeIdentifier(TypeIdentifier parentType, System.Type systemType)
        {
            parentNamespaceID = parentType.parentNamespaceID;
            parentTypeID = parentType.TypeID;
            this.systemType = systemType;
        }

        internal TypeIdentifier(string typeID)
        {
            string[] typeIDParts = typeID.Split(':');
            string[] typeIDParts2 = typeIDParts[1].Split('.');
            parentNamespaceID = typeIDParts[0];
            systemType = System.Type.GetType($"{typeIDParts[0]}.{typeIDParts[1]}");
            parentTypeID = typeIDParts2.Length > 1 ? $"{typeIDParts[0]}:{typeID.Substring(0, typeID.Length - typeIDParts2[typeIDParts2.Length - 1].Length - 1)}" : null;
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
        #endregion
    }
}
