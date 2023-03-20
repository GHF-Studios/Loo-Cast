using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IObjectDataRegistry<KeyType, ValueType> : ICSharpInstanceDataRegistry<KeyType, ValueType> where KeyType : IObjectDataIdentifier where ValueType : IObjectDataIdentifiable
    {
        
    }
}
