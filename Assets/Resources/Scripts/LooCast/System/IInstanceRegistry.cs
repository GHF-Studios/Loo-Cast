using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IInstanceRegistry<KeyType, ValueType> : IRegistry<KeyType, ValueType> where KeyType : IInstanceIdentifier where ValueType : IInstanceIdentifiable
    {
        
    }
}
