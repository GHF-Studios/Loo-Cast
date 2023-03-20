using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IGameObjectDataRegistry<KeyType, ValueType> : IUnityInstanceDataRegistry<KeyType, ValueType> where KeyType : IGameObjectDataIdentifier where ValueType : IGameObjectDataIdentifiable
    {
        
    }
}
