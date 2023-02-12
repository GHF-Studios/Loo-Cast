using System;
using UnityEngine;

namespace LooCast.Identifier
{
    [Serializable]
    public struct InstanceIdentifier : IIdentifier
    {
        #region Properties
        public TypeIdentifier InstanceType => instanceType;
        public string InstanceID => instanceID;
        public string ID => $"{InstanceType.ID}[{InstanceID}]";
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier instanceType;
        [SerializeField] private string instanceID;
        #endregion

        #region Constructors
        internal InstanceIdentifier(TypeIdentifier instanceType)
        {
            this.instanceType = instanceType;
            instanceID = new Guid().ToString();
        }
        #endregion

        #region Overrides
        // TODO: Implement overrides
        #endregion
    }
}
