﻿using System;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    public class SystemObject
    {
        #region Properties
        public SystemObjectIdentifier Identifier => identifier;

        public Guid SystemObjectInstanceGUID => systemObjectInstanceGUID;
        public object SystemObjectInstance => systemObjectInstance;

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

        private Guid systemObjectInstanceGUID;
        private object systemObjectInstance;

        private Type containingType;

#nullable enable
        private SystemObject? parentSystemObject;
#nullable disable
        private SystemObjectRegistry childSystemObjects;
        #endregion

        #region Constructors
        public SystemObject(Guid systemObjectInstanceGUID, object systemObjectInstance, Type containingType, SystemObject parentSystemObject = null)
        {
            identifier = new SystemObjectIdentifier(ContainingType.Identifier, SystemObjectInstanceGUID);
            
            this.systemObjectInstanceGUID = systemObjectInstanceGUID;
            this.systemObjectInstance = systemObjectInstance;

            this.containingType = containingType;

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
