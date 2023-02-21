using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface INamespaceRegistry<KeyType, ValueType> : IRegistry<KeyType, ValueType> where KeyType : INamespaceIdentifier where ValueType : INamespaceIdentifiable
    {
        
    }
}
