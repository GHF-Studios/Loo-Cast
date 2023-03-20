using System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [Serializable]
    public class InstanceIdentifier : IInstanceIdentifier, IInstanceTypeIdentifier, INamespaceIdentifier
    {
        #region Properties
        public string InstanceTypeID => instanceTypeID;
        public string InstanceGUID => instanceGUID;
        public string InstanceID => $"{instanceTypeID}[{instanceGUID}]";
        public string ID => InstanceID;
        #endregion

        #region Fields
        [SerializeField] protected string instanceTypeID;
        [SerializeField] protected string instanceGUID;
        #endregion

        #region Constructors
        public InstanceIdentifier(ITypeIdentifier instanceTypeIdentifier, string instanceGUID)
        {
            instanceTypeID = instanceTypeIdentifier.TypeID;
            this.instanceGUID = instanceGUID;
        }

        public InstanceIdentifier(string instanceTypeID, string instanceGUID)
        {
            this.instanceTypeID = instanceTypeID;
            this.instanceGUID = instanceGUID;
        }

        protected InstanceIdentifier(string instanceID)
        {
            string[] instanceIDParts = instanceID.Split(new char[] { '[', '('});
            instanceTypeID = instanceIDParts[0];
            instanceGUID = instanceIDParts[1].Substring(0, instanceIDParts[1].Length - 1);
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return InstanceID;
        }

        public override int GetHashCode()
        {
            return InstanceID.GetHashCode();
        }

        public bool Equals(IInstanceIdentifier other)
        {
            return InstanceID == other.InstanceID;
        }
        #endregion
    }
}
