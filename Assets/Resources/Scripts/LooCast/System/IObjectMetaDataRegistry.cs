using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IObjectMetaDataRegistry<KeyType, ValueType> : ICSharpInstanceMetaDataRegistry<KeyType, ValueType> where KeyType : IObjectMetaDataIdentifier where ValueType : IObjectMetaDataIdentifiable
    {
        
    }
}
