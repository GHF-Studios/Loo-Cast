using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IResourceObjectRegistry<KeyType, ValueType> : IResourceRegistry<KeyType, ValueType> where KeyType : IResourceObjectIdentifier where ValueType : IResourceObjectIdentifiable
    {
        
    }
}
