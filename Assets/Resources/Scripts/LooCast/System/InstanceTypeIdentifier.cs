using CSSystem = System;
using UnityEngine;

namespace LooCast.System.Identification
{
    public class InstanceTypeIdentifier : TypeIdentifier, IInstanceTypeIdentifier
    {
        #region Properties
        public string ParentInstanceTypeID => parentInstanceTypeID;
        public string InstanceTypeID => parentInstanceTypeID == null ? $"{parentNamespaceID}:{cssystemType.FullName}" : $"{parentTypeID}.{cssystemType.FullName}";
        #endregion

        #region Fields
        [SerializeField] private string parentInstanceTypeID;
        #endregion

        #region Constructors
        public InstanceTypeIdentifier(NamespaceIdentifier parentNamespace, CSSystem.Type systemType) : base(parentNamespace, systemType)
        {
            
        }

        public InstanceTypeIdentifier(InstanceTypeIdentifier parentInstanceType, CSSystem.Type systemType) : base(parentInstanceType, systemType)
        {
        }

        public InstanceTypeIdentifier(string instanceTypeID) : base(instanceTypeID)
        {
        }
        #endregion
    }
}
