using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;

    public class SystemObject : ILooCastObject
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

        #region Static Methods
#nullable enable
        public static T Create<T>(SystemObject? parentSystemObject = null) where T : SystemObject
        {
            T systemObject = Activator.CreateInstance<T>();
            systemObject.systemObjectInstanceGUID = Guid.NewGuid();
            systemObject.systemObjectType = new Type<T>();
            systemObject.systemObjectIdentifier = new SystemObjectIdentifier(systemObject.systemObjectType.TypeIdentifier, systemObject.systemObjectInstanceGUID);
            systemObject.parentSystemObject = parentSystemObject;
            systemObject.childSystemObjects = new HashSet<SystemObject>();
            systemObject.OnPreConstruct();
            systemObject.OnConstruct();
            systemObject.OnPostConstruct();
            return systemObject;
        }
#nullable disable
        #endregion

        #region Methods
        protected virtual void OnPreConstruct()
        {
            
        }

        protected virtual void OnConstruct()
        {
            
        }

        protected virtual void OnPostConstruct()
        {
            
        }
        #endregion
    }
}
