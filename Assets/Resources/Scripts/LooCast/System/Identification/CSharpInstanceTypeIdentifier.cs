using CSSystem = System;
using UnityEngine;

namespace LooCast.System.Identification
{
    public class CSharpInstanceTypeIdentifier : InstanceTypeIdentifier, ICSharpInstanceTypeIdentifier
    {
        #region Properties
        public string ParentCSharpInstanceTypeID => parentCSharpInstanceTypeID;
        public string CSharpInstanceTypeID => parentCSharpInstanceTypeID == null ? $"{parentNamespaceID}:{cssystemType.FullName}" : $"{parentCSharpInstanceTypeID}.{cssystemType.FullName}";
        #endregion

        #region Fields
        [SerializeField] private string parentCSharpInstanceTypeID;
        #endregion

        #region Constructors
        public CSharpInstanceTypeIdentifier(NamespaceIdentifier parentNamespace, CSSystem.Type systemType) : base(parentNamespace, systemType)
        {

        }

        public CSharpInstanceTypeIdentifier(CSharpInstanceTypeIdentifier parentCSharpInstanceType, CSSystem.Type systemType) : base(parentCSharpInstanceType, systemType)
        {
        }

        public CSharpInstanceTypeIdentifier(string csharpInstanceTypeID) : base(csharpInstanceTypeID)
        {
        }
        #endregion
    }
}
