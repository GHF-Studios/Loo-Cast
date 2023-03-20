using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IResourceFileDataRegistry<KeyType, ValueType> : IResourceObjectDataRegistry<KeyType, ValueType> where KeyType : IResourceFileDataIdentifier where ValueType : IResourceFileDataIdentifiable
    {
        
    }
}
