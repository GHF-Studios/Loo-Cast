using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IObjectRegistry<KeyType, ValueType> : ICSharpInstanceRegistry<KeyType, ValueType> where KeyType : IObjectIdentifier where ValueType : IObjectIdentifiable
    {
        
    }
}
