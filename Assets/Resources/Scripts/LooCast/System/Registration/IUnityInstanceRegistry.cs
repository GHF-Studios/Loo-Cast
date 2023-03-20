using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IUnityInstanceRegistry<KeyType, ValueType> : IRegistry<KeyType, ValueType> where KeyType : IUnityInstanceIdentifier where ValueType : IUnityInstanceIdentifiable
    {
        
    }
}
