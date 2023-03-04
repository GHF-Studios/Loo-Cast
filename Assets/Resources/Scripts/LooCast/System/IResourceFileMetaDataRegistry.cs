using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IResourceFileMetaDataRegistry<KeyType, ValueType> : IResourceObjectMetaDataRegistry<KeyType, ValueType> where KeyType : IResourceFileMetaDataIdentifier where ValueType : IResourceFileMetaDataIdentifiable
    {
        
    }
}
