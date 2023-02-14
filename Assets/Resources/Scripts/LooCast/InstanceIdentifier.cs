using System;
using UnityEngine;

namespace LooCast
{
    [Serializable]
    public record InstanceIdentifier : IGenericIdentifier<Instance>
    {
        #region Properties
        public string InstanceTypeID => instanceTypeID;
        public string InstanceGUID => instanceGUID;
        
        public string InstanceID => $"{instanceTypeID}[{instanceGUID}]";
        public string ID => InstanceID;
        #endregion

        #region Fields
        [SerializeField] private string instanceTypeID;
        [SerializeField] private string instanceGUID;
        #endregion

        #region Constructors
        internal InstanceIdentifier(TypeIdentifier instanceType)
        {
            instanceTypeID = instanceType.TypeID;
            instanceGUID = new Guid().ToString();
        }

        internal InstanceIdentifier(string instanceID)
        {
            string[] instanceIDParts = instanceID.Split('[');
            instanceTypeID = instanceIDParts[0];
            instanceGUID = instanceIDParts[1].Substring(0, instanceIDParts[1].Length - 1);
        }
        #endregion

        #region Operators
        public static implicit operator InstanceIdentifier(string instanceID)
        {
            return new InstanceIdentifier(instanceID);
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (!(obj is InstanceIdentifier))
            {
                return false;
            }
            InstanceIdentifier otherInstanceIdentifier = (InstanceIdentifier)obj;
            return this.Equals(otherInstanceIdentifier);
        }

        public bool Equals(InstanceIdentifier other)
        {
            return InstanceID == other.InstanceID;
        }

        public override int GetHashCode()
        {
            return InstanceID.GetHashCode();
        }

        public static bool operator ==(InstanceIdentifier left, InstanceIdentifier right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(InstanceIdentifier left, InstanceIdentifier right)
        {
            return !(left == right);
        }

        public override string ToString()
        {
            return InstanceID;
        }
        #endregion
    }
}
