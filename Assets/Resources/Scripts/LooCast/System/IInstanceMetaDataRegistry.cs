using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IInstanceMetaDataRegistry<KeyType, ValueType> : IMetaDataRegistry<KeyType, ValueType> where KeyType : IInstanceMetaDataIdentifier where ValueType : IInstanceMetaDataIdentifiable
    {
        
    }
}
