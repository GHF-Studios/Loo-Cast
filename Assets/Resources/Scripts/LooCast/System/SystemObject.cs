﻿using System;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.Registries;

    public class SystemObject : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => systemObjectIdentifier;
        public SystemObjectIdentifier SystemObjectIdentifier => systemObjectIdentifier;

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
        private SystemObjectIdentifier? systemObjectIdentifier;
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
        public SystemObject(TypeIdentifier typeIdentifier, SystemObject parentSystemObject = null)
        {
            TypeManager typeManager = TypeManager.Instance;

            systemObjectInstanceGUID = Guid.NewGuid();
            systemObjectInstance = new object();

            containingType = typeManager.GetType(typeIdentifier);

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
            return SystemObjectIdentifier.Equals(otherSystemObject.SystemObjectIdentifier);
        }

        public override int GetHashCode()
        {
            return SystemObjectIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return SystemObjectIdentifier.ToString();
        }
        #endregion
    }
}
