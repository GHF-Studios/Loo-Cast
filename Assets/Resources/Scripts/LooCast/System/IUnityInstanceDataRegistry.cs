using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IUnityInstanceDataRegistry<KeyType, ValueType> : IInstanceDataRegistry<KeyType, ValueType> where KeyType : IUnityInstanceDataIdentifier where ValueType : IUnityInstanceDataIdentifiable
    {
        
    }
}
