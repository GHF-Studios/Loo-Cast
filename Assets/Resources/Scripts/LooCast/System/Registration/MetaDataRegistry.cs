using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class MetaDataRegistry : Registry<IMetaDataIdentifier, IMetaDataIdentifiable>, IMetaDataRegistry<IMetaDataIdentifier, IMetaDataIdentifiable>
    {
        public MetaDataRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
