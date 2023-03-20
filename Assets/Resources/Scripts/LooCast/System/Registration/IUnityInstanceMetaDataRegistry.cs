using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IUnityInstanceMetaDataRegistry<KeyType, ValueType> : IInstanceMetaDataRegistry<KeyType, ValueType> where KeyType : IUnityInstanceMetaDataIdentifier where ValueType : IUnityInstanceMetaDataIdentifiable
    {
        
    }
}
