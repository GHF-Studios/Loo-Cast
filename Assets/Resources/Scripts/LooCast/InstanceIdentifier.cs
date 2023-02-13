﻿using System;
using UnityEngine;

namespace LooCast
{
    [Serializable]
    public struct InstanceIdentifier : IIdentifier
    {
        #region Properties
        public TypeIdentifier InstanceType => instanceType;
        public string InstanceGUID => instanceGUID;
        public string InstanceID => $"{instanceType.ID}[{instanceGUID}]";
        public string ID => InstanceID;
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier instanceType;
        [SerializeField] private string instanceGUID;
        #endregion

        #region Constructors
        internal InstanceIdentifier(TypeIdentifier instanceType)
        {
            this.instanceType = instanceType;
            instanceGUID = new Guid().ToString();
        }

        internal InstanceIdentifier(string instanceID)
        {
            string[] instanceIDParts = instanceID.Split('[');
            instanceType = new TypeIdentifier(instanceIDParts[0]);
            instanceGUID = instanceIDParts[1].Substring(0, instanceIDParts[1].Length - 1);
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
        #endregion
    }
}
