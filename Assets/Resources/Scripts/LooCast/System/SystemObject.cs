using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.MetaData;

    public abstract class SystemObject : ILooCastObject
    {
        #region Properties
        public Identifier Identifier => systemObjectMetaData.SystemObjectIdentifier;
        public SystemObjectMetaData SystemObjectMetaData => systemObjectMetaData;
        #endregion

        #region Fields
        private SystemObjectMetaData systemObjectMetaData;
        #endregion

        #region Static Methods
#nullable enable
        public static SystemObjectType CreateSystemObject<SystemObjectType, SystemObjectMetaDataType>(SystemObjectMetaDataType? systemObjectMetaData = default(SystemObjectMetaDataType)) 
            where SystemObjectType : SystemObject, new() 
            where SystemObjectMetaDataType : SystemObjectMetaData, new()
        {
            if (systemObjectMetaData == null)
            {
                return CreateSystemObject<SystemObjectType>();
            }
            
            SystemObjectType systemObject = Activator.CreateInstance<SystemObjectType>();
            systemObject.SetMetaData(systemObjectMetaData);
            systemObject.PreConstruct();
            systemObject.Construct();
            systemObject.PostConstruct();
            return systemObject;
        }
#nullable disable

        public static SystemObjectType CreateSystemObject<SystemObjectType>()
            where SystemObjectType : SystemObject, new()
        {
            SystemObjectType systemObject = Activator.CreateInstance<SystemObjectType>();
            SystemObjectMetaData systemObjectMetaData = Activator.CreateInstance<SystemObjectMetaData>();
            systemObject.CreateMetaData<SystemObjectType, SystemObjectMetaData>(ref systemObjectMetaData);
            systemObject.SetMetaData(systemObjectMetaData);
            systemObject.PreConstruct();
            systemObject.Construct();
            systemObject.PostConstruct();
            return systemObject;
        }
        #endregion

        #region Methods
        protected virtual void CreateMetaData<SystemObjectType, SystemObjectMetaDataType>(ref SystemObjectMetaDataType systemObjectMetaData)
            where SystemObjectType : SystemObject, new()
            where SystemObjectMetaDataType : SystemObjectMetaData, new()
        {
            systemObjectMetaData.SystemObjectIdentifier = new SystemObjectIdentifier(TypeManager.Instance.GetType<SystemObjectType>().TypeIdentifier, Guid.NewGuid());
            systemObjectMetaData.ParentSystemObject = null;
            systemObjectMetaData.ChildSystemObjects = new List<SystemObject>();
        }

        public virtual void SetMetaData(SystemObjectMetaData systemObjectMetaData)
        {
            this.systemObjectMetaData = systemObjectMetaData;
        }

        protected virtual void PreConstruct()
        {
            
        }

        protected virtual void Construct()
        {
            
        }

        protected virtual void PostConstruct()
        {
            
        }
        #endregion
    }
}
