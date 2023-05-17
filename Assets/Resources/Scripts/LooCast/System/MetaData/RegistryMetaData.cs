using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System;
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    [Serializable]
    public sealed class RegistryMetaData<KeyType, ValueType> : MetaData, IRegistryMetaData
        where KeyType : IIdentifier
        where ValueType : IIdentifiable
    {
        #region Properties
        public override IIdentifier ObjectIdentifier => RegistryIdentifier;
        public IRegistryIdentifier RegistryIdentifier => registryIdentifier;

        public override HierarchyElement ObjectHierarchyElement => objectHierarchyElement;

        public override IMetaData MetaDataParent => RegistryMetaDataParent;
        public IRegistryMetaData RegistryMetaDataParent => registryMetaDataParent;

        public IType RegistryKeyType => registryKeyType;
        public IType RegistryValueType => registryValueType;
        #endregion

        #region Fields
        private RegistryIdentifier registryIdentifier;
        
        private HierarchyElement objectHierarchyElement;
        
        private IRegistryMetaData registryMetaDataParent;
        
        private Type<KeyType> registryKeyType;
        private Type<ValueType> registryValueType;
        #endregion

        #region Constructors
        public RegistryMetaData(HierarchyElement objectHierarchyElement, string gusid, string gusp, string gusidParent, string guspParent) : base(gusid, gusp, gusidParent, guspParent)
        {
            this.objectHierarchyElement = objectHierarchyElement;
        }
        #endregion

        #region Overrides
        public override bool Validate()
        {
            if(!base.Validate())
            {
                return false;
            }

            if (objectHierarchyElement is not HierarchyFolder && objectHierarchyElement is not HierarchyFile && objectHierarchyElement is not HierarchyObject)
            {
                return false;
            }

            return true;
        }

        public override void PreInitialize()
        {
            base.PreInitialize();

            TypeRegistry typeRegistry = MainManager.Instance.MainRegistry.GetRegistry(typeof(IType)) as TypeRegistry;
            MetaDataRegistry metaDataRegistry = MainManager.Instance.MainRegistry.GetRegistry(typeof(IRegistry)) as MetaDataRegistry;

            registryIdentifier = Identifiers.RegistryIdentifier.Parse(gusid);
            registryMetaDataParent = metaDataRegistry.GetValue((RegistryIdentifier)gusidParent) as IRegistryMetaData;
            registryKeyType = (Type<KeyType>)typeRegistry.GetValue(typeof(KeyType));
            registryValueType = (Type<ValueType>)typeRegistry.GetValue(typeof(ValueType));
        }
        #endregion
    }
}
