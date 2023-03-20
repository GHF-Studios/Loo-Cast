using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IComponentDataRegistry<KeyType, ValueType> : IGameObjectDataRegistry<KeyType, ValueType> where KeyType : IComponentDataIdentifier where ValueType : IComponentDataIdentifiable
    {
        
    }
}
