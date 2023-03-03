using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IResourceFileRegistry<KeyType, ValueType> : IResourceObjectRegistry<KeyType, ValueType> where KeyType : IResourceFileIdentifier where ValueType : IResourceFileIdentifiable
    {
        
    }
}
