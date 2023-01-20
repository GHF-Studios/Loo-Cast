using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    [Serializable]
    public class InstanceIdentifier : IIdentifiableInstance
    {
        public TypeIdentifier Type { get; set; }
        public long InstanceID { get; private set; }
        public string TypeName { get { return Type.TypeName; } }
        public string GUID { get; private set; }

        private static long _currentGUID = 0;

        public InstanceIdentifier(TypeIdentifier type, long instanceID)
        {
            Type = type;
            InstanceID = _currentGUID++;
            GUID = $"{TypeName}[{InstanceID}]";
        }

        #region Methods
        public override bool Equals(object obj)
        {
            if (obj == null) return false;
            InstanceIdentifier objAsPart = obj as InstanceIdentifier;
            if (objAsPart == null) return false;
            else return Equals(objAsPart);
        }

        public bool Equals(InstanceIdentifier other)
        {
            if (other == null) return false;
            return (this.Type.GUID.Equals(other.Type.GUID) && this.InstanceID.Equals(other.InstanceID));
        }

        public override int GetHashCode()
        {
            return Type.GetHashCode() ^ InstanceID.GetHashCode();
        }
        #endregion
    }
}