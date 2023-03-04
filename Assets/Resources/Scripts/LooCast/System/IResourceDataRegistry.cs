using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IResourceDataRegistry<KeyType, ValueType> : IObjectDataRegistry<KeyType, ValueType> where KeyType : IResourceDataIdentifier where ValueType : IResourceDataIdentifiable
    {
        
    }
}
