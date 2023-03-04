using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IGameObjectMetaDataRegistry<KeyType, ValueType> : IUnityInstanceMetaDataRegistry<KeyType, ValueType> where KeyType : IGameObjectMetaDataIdentifier where ValueType : IGameObjectMetaDataIdentifiable
    {
        
    }
}
