using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IResourceMetaDataRegistry<KeyType, ValueType> : IObjectMetaDataRegistry<KeyType, ValueType> where KeyType : IResourceMetaDataIdentifier where ValueType : IResourceMetaDataIdentifiable
    {
        
    }
}
