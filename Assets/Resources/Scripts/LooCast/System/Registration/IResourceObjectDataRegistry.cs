using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IResourceObjectDataRegistry<KeyType, ValueType> : IResourceDataRegistry<KeyType, ValueType> where KeyType : IResourceObjectDataIdentifier where ValueType : IResourceObjectDataIdentifiable
    {
        
    }
}
