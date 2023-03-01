using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IMetaDataRegistry<KeyType, ValueType> : IObjectRegistry<KeyType, ValueType> where KeyType : IMetaDataIdentifier where ValueType : IMetaDataIdentifiable
    {
        
    }
}
