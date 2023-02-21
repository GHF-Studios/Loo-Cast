using CSSystem = System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [CSSystem.Serializable]
    public class CSharpInstanceIdentifier : InstanceIdentifier, ICSharpInstanceIdentifier
    {
        #region Properties
        public string CSharpInstanceTypeID => csharpInstanceTypeID;
        public string CSharpInstanceGUID => csharpInstanceGUID;
        public string CSharpInstanceID => $"{csharpInstanceTypeID}[{csharpInstanceGUID}]";
        #endregion

        #region Fields
        [SerializeField] protected string csharpInstanceTypeID;
        [SerializeField] protected string csharpInstanceGUID;

        public CSharpInstanceIdentifier(ICSharpInstanceTypeIdentifier csharpInstanceTypeIdentifier) : base(csharpInstanceTypeIdentifier, new CSSystem.Guid().ToString())
        {
            csharpInstanceTypeID = csharpInstanceTypeIdentifier.TypeID;
            csharpInstanceGUID = instanceGUID;
        }

        public CSharpInstanceIdentifier(string csharpInstanceTypeID, string csharpInstanceGUID) : base(csharpInstanceTypeID, csharpInstanceGUID)
        {
            csharpInstanceTypeID = instanceTypeID;
            csharpInstanceGUID = instanceGUID;
        }

        public CSharpInstanceIdentifier(string csharpInstanceID) : base(csharpInstanceID)
        {
            string[] split = csharpInstanceID.Split('(');
            csharpInstanceTypeID = split[0];
            csharpInstanceGUID = split[1].Substring(0, split[1].Length - 1);
        }
        #endregion

        #region Operators
        public static implicit operator CSharpInstanceIdentifier(string instanceID)
        {
            return new CSharpInstanceIdentifier(instanceID);
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (!(obj is ICSharpInstanceIdentifier))
            {
                return false;
            }
            ICSharpInstanceIdentifier otherCSharpInstanceIdentifier = (ICSharpInstanceIdentifier)obj;
            return this.Equals(otherCSharpInstanceIdentifier);
        }

        public static bool operator ==(CSharpInstanceIdentifier left, ICSharpInstanceIdentifier right)
        {
            return left.Equals(right);
        }

        public static bool operator ==(ICSharpInstanceIdentifier left, CSharpInstanceIdentifier right)
        {
            return right.Equals(left);
        }

        public static bool operator !=(CSharpInstanceIdentifier left, ICSharpInstanceIdentifier right)
        {
            return !(left == right);
        }

        public static bool operator !=(ICSharpInstanceIdentifier left, CSharpInstanceIdentifier right)
        {
            return !(left == right);
        }
        #endregion
    }
}
