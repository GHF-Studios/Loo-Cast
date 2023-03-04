using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IResourceObjectMetaDataRegistry<KeyType, ValueType> : IResourceMetaDataRegistry<KeyType, ValueType> where KeyType : IResourceObjectMetaDataIdentifier where ValueType : IResourceObjectMetaDataIdentifiable
    {
        
    }
}
