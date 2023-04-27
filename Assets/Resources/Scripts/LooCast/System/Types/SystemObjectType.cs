using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;

namespace LooCast.System.Types
{
    using LooCast.System.Data;
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;

    public abstract class SystemObjectType<TInstance> : InstanceType<TInstance>, ISystemObjectType
        where TInstance : SystemObjectType<TInstance>.SystemObject, new()
    {
        #region Classes
        public abstract class SystemObject : ISystemObjectType.ISystemObject
        {
            #region Properties
            public abstract IMetaData MetaData { get; set; }
            public abstract IInstanceMetaData InstanceMetaData { get; set; }
            public abstract SystemObjectMetaData SystemObjectMetaData { get; set; }

            public abstract IData Data { get; set; }
            public abstract IInstanceData InstanceData { get; set; }
            public abstract SystemObjectData SystemObjectData { get; set; }
            #endregion

            #region Static Methods
#nullable enable
            public static SystemObjectType CreateSystemObject<SystemObjectType, SystemObjectMetaDataType>(SystemObjectMetaDataType? systemObjectMetaData = default(SystemObjectMetaDataType))
                where SystemObjectType : SystemObject, new()
                where SystemObjectMetaDataType : SystemObjectMetaData, new()
            {
                if (systemObjectMetaData == null)
                {
                    systemObjectMetaData = Activator.CreateInstance<SystemObjectMetaDataType>();
                }

                SystemObjectType systemObject = Activator.CreateInstance<SystemObjectType>();
                
                systemObject.CreateMetaData<SystemObjectType, SystemObjectMetaDataType>(ref systemObjectMetaData);
                
                systemObject.SetMetaData(systemObjectMetaData);
                
                systemObject.PreConstruct();
                systemObject.Construct();
                systemObject.PostConstruct();
                
                return systemObject;
            }
#nullable disable
            #endregion

            #region Methods
            public abstract bool Validate();

            protected virtual void CreateMetaData<SystemObjectType, SystemObjectMetaDataType>(ref SystemObjectMetaDataType systemObjectMetaData)
                where SystemObjectType : SystemObject, new()
                where SystemObjectMetaDataType : SystemObjectMetaData, new()
            {
                systemObjectMetaData.SystemObjectIdentifier = new SystemObjectIdentifier(TypeManager.Instance.GetType<SystemObjectType>().TypeIdentifier, Guid.NewGuid());
                systemObjectMetaData.ParentSystemObject = null;
                systemObjectMetaData.ChildSystemObjects = new List<SystemObjectType>();
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
        #endregion

        #region Properties
        public abstract SystemObjectTypeMetaData SystemObjectTypeMetaData { get; set; }

        public abstract SystemObjectTypeData SystemObjectTypeData { get; set; }
        #endregion
    }
}
