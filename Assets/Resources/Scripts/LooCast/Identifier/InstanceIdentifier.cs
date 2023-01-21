using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    [Serializable]
    public class InstanceIdentifier : IIdentifiableInstance
    {
        #region Properties
        public string ID
        {
            get
            {
                return $"{InstanceType.ID}.{InstanceID}";
            }
        }

        public IIdentifiableType InstanceType => instanceType;
        public long InstanceID => instanceID;
        #endregion

        #region Fields
        [SerializeField] private IIdentifiableType instanceType;
        [SerializeField] private long instanceID;
        #endregion
    }
}