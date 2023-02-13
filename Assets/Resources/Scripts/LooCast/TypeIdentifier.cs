using System;
using UnityEngine;

namespace LooCast
{
    [Serializable]
    public struct TypeIdentifier : IIdentifier
    {
        #region Properties
        public NamespaceIdentifier TypeNamespace => typeNamespace;
        public System.Type SystemType => systemType;
        public string TypeID => typeNamespace.NamespaceID + ":" + systemType.Name;
        public string ID => TypeID;
        #endregion

        #region Fields
        [SerializeField] private NamespaceIdentifier typeNamespace;
        [SerializeField] private System.Type systemType;
        #endregion

        #region Constructors
        internal TypeIdentifier(NamespaceIdentifier typeNamespace, System.Type systemType)
        {
            this.typeNamespace = typeNamespace;
            this.systemType = systemType;
        }

        internal TypeIdentifier(string typeID)
        {
            string[] typeIDParts = typeID.Split(':');
            typeNamespace = new NamespaceIdentifier(typeIDParts[0]);
            systemType = System.Type.GetType(typeID.Replace(":", "."));
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
