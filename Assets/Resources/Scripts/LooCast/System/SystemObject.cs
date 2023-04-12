using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.MetaData;

    public class SystemObject : ILooCastObject
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
        public static ObjectType CreateObject<ObjectType, MetaDataType>(MetaDataType? metaData = default(MetaDataType)) 
            where ObjectType : SystemObject, new() 
            where MetaDataType : SystemObjectMetaData, new()
        {
            ObjectType systemObject = Activator.CreateInstance<ObjectType>();
            if (metaData == null)
            {
                metaData = Activator.CreateInstance<MetaDataType>();
                systemObject.CreateMetaData<ObjectType, MetaDataType>(ref metaData);
            }
            systemObject.SetMetaData(metaData);
            systemObject.PreConstruct();
            systemObject.Construct();
            systemObject.PostConstruct();
            return systemObject;
        }
#nullable disable
        #endregion

        #region Methods
        protected virtual void CreateMetaData<ObjectType, MetaDataType>(ref MetaDataType metaData)
            where ObjectType : SystemObject, new()
            where MetaDataType : SystemObjectMetaData, new()
        {
            metaData.SystemObjectIdentifier = new SystemObjectIdentifier(new Type<ObjectType>().TypeIdentifier, Guid.NewGuid());
            metaData.ParentSystemObject = null;
            metaData.ChildSystemObjects = new List<SystemObject>();
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
