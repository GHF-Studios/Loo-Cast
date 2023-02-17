using CSSystem = System;
using UnityEngine;

namespace LooCast.System.Identification
{
    public class UnityInstanceTypeIdentifier : InstanceTypeIdentifier, IUnityInstanceTypeIdentifier
    {
        #region Properties
        public string ParentUnityInstanceTypeID => parentUnityInstanceTypeID;
        public string UnityInstanceTypeID => parentUnityInstanceTypeID == null ? $"{parentNamespaceID}:{cssystemType.FullName}" : $"{parentUnityInstanceTypeID}.{cssystemType.FullName}";
        #endregion

        #region Fields
        [SerializeField] private string parentUnityInstanceTypeID;
        #endregion

        #region Constructors
        public UnityInstanceTypeIdentifier(NamespaceIdentifier parentNamespace, CSSystem.Type systemType) : base(parentNamespace, systemType)
        {

        }

        public UnityInstanceTypeIdentifier(UnityInstanceTypeIdentifier parentUnityInstanceType, CSSystem.Type systemType) : base(parentUnityInstanceType, systemType)
        {
        }

        public UnityInstanceTypeIdentifier(string unityInstanceTypeID) : base(unityInstanceTypeID)
        {
        }
        #endregion
    }
}
