using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;

    public class SystemObject : IHierarchyElement
    {
        #region Properties
        public Identifier Identifier => systemObjectIdentifier;
        public SystemObjectIdentifier SystemObjectIdentifier => systemObjectIdentifier;

        public Guid SystemObjectInstanceGUID => systemObjectInstanceGUID;

        public Type SystemObjectType => systemObjectType;
#nullable enable
        public SystemObject? ParentSystemObject => parentSystemObject;
#nullable disable
        public HashSet<SystemObject> ChildSystemObjects => childSystemObjects;
        #endregion

        #region Fields
        private SystemObjectIdentifier systemObjectIdentifier;

        private Guid systemObjectInstanceGUID;

        private Type systemObjectType;
#nullable enable
        private SystemObject? parentSystemObject;
#nullable disable
        private HashSet<SystemObject> childSystemObjects;
        #endregion

        #region Constructors
#nullable enable
        public SystemObject(Type systemObjectType, SystemObject? parentSystemObject = null)
        {
            this.systemObjectType = systemObjectType;
            this.parentSystemObject = parentSystemObject;

            childSystemObjects = new HashSet<SystemObject>();

            systemObjectIdentifier = new SystemObjectIdentifier(systemObjectType.TypeIdentifier, Guid.NewGuid());
            systemObjectInstanceGUID = systemObjectIdentifier.SystemObjectInstanceGUID;
        }
#nullable disable
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
