using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Core.MetaData
{
    using LooCast.Core.Identifiers;
    using LooCast.Core.Types;

    [Serializable]
    public abstract class SystemObjectMetaData : InstanceMetaData, ISystemObjectMetaData
    {
        #region Properties
        public abstract ISystemObjectIdentifier SystemObjectIdentifier { get; }
        public abstract ISystemObjectTypeMetaData SystemObjectTypeMetaData { get; }
        public abstract ISystemObjectMetaData ParentSystemObjectMetaData { get; }
        public abstract IEnumerable<ISystemObjectMetaData> ChildSystemObjectsMetaData { get; }

        public ISystemObjectType SystemObjectType { get; private set; }
        public ISystemObjectType.ISystemObject ParentSystemObject { get; private set; }
        public IEnumerable<ISystemObjectType.ISystemObject> ChildSystemObjects { get; private set; }
        #endregion

        #region Methods
        public override void PreInitialize()
        {
            base.PreInitialize();

            SystemObjectType = MainManager.Instance.MainRegistry.TypeRegistry.GetValue(SystemObjectTypeMetaData.TypeIdentifier) as ISystemObjectType;
            ParentSystemObject = MainManager.Instance.MainRegistry.SystemObjectRegistry.GetValue(ParentSystemObjectMetaData.SystemObjectIdentifier);
            ChildSystemObjects = MainManager.Instance.MainRegistry.SystemObjectRegistry.GetValues(ChildSystemObjectsMetaData.Select(x => x.SystemObjectIdentifier));
        }
        #endregion
    }
}
