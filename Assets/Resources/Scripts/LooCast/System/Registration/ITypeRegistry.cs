using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface ITypeRegistry<KeyType, ValueType> : IRegistry<KeyType, ValueType> where KeyType : ITypeIdentifier where ValueType : ITypeIdentifiable
    {
        
    }
}
