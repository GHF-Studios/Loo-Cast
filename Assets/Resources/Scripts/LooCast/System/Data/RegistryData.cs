using System;
using System.Collections;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System;
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;
    using LooCast.System.Registries;
    
    [Serializable]
    public sealed class RegistryData<KeyType, ValueType> : Data, IRegistryData
        where KeyType : IObjectIdentifier
        where ValueType : IIdentifiableObject
    {
        #region Properties
        public override IDataIdentifier DataIdentifier => dataIdentifier;

        public override IData DataParent => RegistryDataParent;
        public IRegistryData RegistryDataParent => registryDataParent;
        
        public IDictionary<KeyType, ValueType> Dictionary => dictionary;
        #endregion

        #region Fields
        private DataIdentifier dataIdentifier;
        
        private IRegistryData registryDataParent;
        
        private Dictionary<KeyType, ValueType> dictionary;
        #endregion

        #region Constructors
        public RegistryData(string gusid, string gusp, string gusidParent, string guspParent) : base(gusid, gusp, gusidParent, guspParent)
        {
        }
        #endregion

        #region Overrides
        public override bool Validate()
        {
            if (!base.Validate())
            {
                return false;
            }

            return true;
        }
        
        public override void PreInitialize()
        {
            base.PreInitialize();

            MetaDataRegistry metaDataRegistry = MainManager.Instance.MainRegistry.GetRegistry(typeof(IRegistry)) as MetaDataRegistry;

            containingRegistryMetaData = metaDataRegistry.Get(ContainingMetaDataIdentifier) as IRegistryMetaData;

            dictionary = new Dictionary<KeyType, ValueType>();
        }
        #endregion
    }
}
