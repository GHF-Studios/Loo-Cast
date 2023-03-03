using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IResourceRegistry<KeyType, ValueType> : IObjectRegistry<KeyType, ValueType> where KeyType : IResourceIdentifier where ValueType : IResourceIdentifiable
    {
        
    }
}
