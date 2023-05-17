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
        public IRegistryIdentifier RegistryIdentifier => throw new NotImplementedException();

        public override IMetaData MetaDataParent => RegistryMetaDataParent;
        public IRegistryMetaData RegistryMetaDataParent => registryMetaDataParent;

        public IType RegistryKeyType => registryKeyType;
        public IType RegistryValueType => registryValueType;

        #endregion

        #region Fields
        private RegistryIdentifier registryIdentifier;
        
        private HierarchyElement registryHierarchyElement;
        
        private IRegistryMetaData registryMetaDataParent;
        
        private Type<KeyType> registryKeyType;
        private Type<ValueType> registryValueType;
        #endregion

        #region Constructors
        public RegistryMetaData(HierarchyElement registryHierarchyElement, string gusid, string gusp, string gusidParent, string guspParent) : base(gusid, gusp, gusidParent, guspParent)
        {
            this.registryHierarchyElement = registryHierarchyElement;
        }
        #endregion

        #region Overrides
        public override bool Validate()
        {
            if(!base.Validate())
            {
                return false;
            }

            if (registryHierarchyElement is not HierarchyFolder && registryHierarchyElement is not HierarchyFile && registryHierarchyElement is not HierarchyObject)
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
