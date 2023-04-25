using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;

namespace LooCast.System.Types
{
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;

    public abstract class SystemObjectType<TInstance> : Type<TInstance>
        where TInstance : SystemObjectType<TInstance>.Instance, new()
    {
        #region Classes
        public abstract class Instance : IType.IInstance
        {
            #region Properties
            public IIdentifier Identifier => InstanceIdentifier;
            public IInstanceIdentifier InstanceIdentifier => SystemObjectIdentifier;
            public SystemObjectIdentifier SystemObjectIdentifier => systemObjectMetaData.SystemObjectIdentifier;
            
            public IType Type => InstanceType;
            public IInstanceType InstanceType => SystemObjectType;
            public abstract SystemObjectType<TInstance> SystemObjectType { get; }

            public IMetaData MetaData => InstanceMetaData;
            public IInstanceMetaData InstanceMetaData => SystemObjectMetaData;
            public SystemObjectMetaData SystemObjectMetaData => systemObjectMetaData;

            public IData Data => InstanceData;
            public IInstanceData InstanceData => SystemObjectData;
            public abstract SystemObjectData SystemObjectData { get; }
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
        #endregion

        #region Constructors
        #endregion
    }
}
