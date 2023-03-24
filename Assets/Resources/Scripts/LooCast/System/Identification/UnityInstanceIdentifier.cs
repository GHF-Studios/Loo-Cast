using CSSystem = System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [CSSystem.Serializable]
    public class UnityInstanceIdentifier : InstanceIdentifier, IUnityInstanceIdentifier
    {
        #region Properties
        public string UnityInstanceTypeID => unityInstanceTypeID;
        public string UnityInstanceGUID => unityInstanceGUID;
        public string UnityInstanceID => $"{unityInstanceTypeID}[{unityInstanceGUID}]";
        #endregion

        #region Fields
        [SerializeField] private string unityInstanceTypeID;
        [SerializeField] private string unityInstanceGUID;

        public UnityInstanceIdentifier(IUnityInstanceTypeIdentifier unityInstanceTypeIdentifier) : base(unityInstanceTypeIdentifier, new CSSystem.Guid().ToString())
        {
            unityInstanceTypeID = unityInstanceTypeIdentifier.TypeID;
            unityInstanceGUID = instanceGUID;
        }

        public UnityInstanceIdentifier(string unityInstanceTypeID, string unityInstanceGUID) : base(unityInstanceTypeID, unityInstanceGUID)
        {
            unityInstanceTypeID = typeID;
            unityInstanceGUID = instanceGUID;
        }

        public UnityInstanceIdentifier(string unityInstanceID) : base(unityInstanceID)
        {
            string[] split = unityInstanceID.Split('[');
            unityInstanceTypeID = split[0];
            unityInstanceGUID = split[1].Substring(0, split[1].Length - 1);
        }
        #endregion

        #region Operators
        public static implicit operator UnityInstanceIdentifier(string instanceID)
        {
            return new UnityInstanceIdentifier(instanceID);
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (!(obj is IUnityInstanceIdentifier))
            {
                return false;
            }
            IUnityInstanceIdentifier otherUnityInstanceIdentifier = (IUnityInstanceIdentifier)obj;
            return this.Equals(otherUnityInstanceIdentifier);
        }

        public override int GetHashCode()
        {
            return ID.GetHashCode();
        }

        public static bool operator ==(UnityInstanceIdentifier left, IUnityInstanceIdentifier right)
        {
            return left.Equals(right);
        }

        public static bool operator ==(IUnityInstanceIdentifier left, UnityInstanceIdentifier right)
        {
            return right.Equals(left);
        }

        public static bool operator !=(UnityInstanceIdentifier left, IUnityInstanceIdentifier right)
        {
            return !(left == right);
        }

        public static bool operator !=(IUnityInstanceIdentifier left, UnityInstanceIdentifier right)
        {
            return !(left == right);
        }
        #endregion
    }
}
