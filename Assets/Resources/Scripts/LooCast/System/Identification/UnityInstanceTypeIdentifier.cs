using CSSystem = System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [CSSystem.Serializable]
    public class UnityInstanceTypeIdentifier : InstanceTypeIdentifier, IUnityInstanceTypeIdentifier
    {
        #region Properties
        public string ParentUnityInstanceTypeID => ParentTypeID;
        public string UnityInstanceTypeID => TypeID;
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
