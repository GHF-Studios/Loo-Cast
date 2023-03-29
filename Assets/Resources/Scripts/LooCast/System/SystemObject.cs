using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Registration;

    public class SystemObject
    {
        #region Properties
        public SystemObjectIdentifier Identifier
        {
            get
            {
                if (identifier == null)
                {
                    identifier = new SystemObjectIdentifier(ContainingType.Identifier, InstanceGUID);
                }
                return identifier.Value;
            }
        }
        
        public Guid InstanceGUID => instanceGUID;
        public object Instance => instance;

        public Type ContainingType => containingType;

#nullable enable
        public SystemObject? ParentSystemObject => parentSystemObject;
#nullable disable
        public SystemObjectRegistry ChildSystemObjects => childSystemObjects;
        #endregion

        #region Fields
#nullable enable
        private SystemObjectIdentifier? identifier;
#nullable disable

        private Guid instanceGUID;
        private object instance;

        private Type containingType;

#nullable enable
        private SystemObject? parentSystemObject;
#nullable disable
        private SystemObjectRegistry childSystemObjects;
        #endregion

        #region Constructors
        public SystemObject(Guid instanceGUID, object instance, Type containingType)
        {
            this.instanceGUID = instanceGUID;
            this.instance = instance;

            this.containingType = containingType;

            parentSystemObject = null;
            childSystemObjects = new SystemObjectRegistry();
        }

        public SystemObject(Guid instanceGUID, object instance, SystemObject parentSystemObject)
        {
            this.instanceGUID = instanceGUID;
            this.instance = instance;

            this.containingType = parentSystemObject.ContainingType;

            this.parentSystemObject = parentSystemObject;
            childSystemObjects = new SystemObjectRegistry();
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is SystemObject otherSystemObject)
            {
                return Equals(otherSystemObject);
            }
            return false;
        }

        public bool Equals(SystemObject otherSystemObject)
        {
            return Identifier.Equals(otherSystemObject.Identifier);
        }

        public override int GetHashCode()
        {
            return Identifier.GetHashCode();
        }

        public override string ToString()
        {
            return Identifier.ToString();
        }
        #endregion
    }
}
