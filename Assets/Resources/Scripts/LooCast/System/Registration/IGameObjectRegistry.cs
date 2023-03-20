using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IGameObjectRegistry<KeyType, ValueType> : IUnityInstanceRegistry<KeyType, ValueType> where KeyType : IGameObjectIdentifier where ValueType : IGameObjectIdentifiable
    {
        
    }
}
