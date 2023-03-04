using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface ICSharpInstanceDataRegistry<KeyType, ValueType> : IInstanceDataRegistry<KeyType, ValueType> where KeyType : ICSharpInstanceDataIdentifier where ValueType : ICSharpInstanceDataIdentifiable
    {
        
    }
}
