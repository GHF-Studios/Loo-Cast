using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IInstanceDataRegistry<KeyType, ValueType> : IDataRegistry<KeyType, ValueType> where KeyType : IInstanceDataIdentifier where ValueType : IInstanceDataIdentifiable
    {
        
    }
}
