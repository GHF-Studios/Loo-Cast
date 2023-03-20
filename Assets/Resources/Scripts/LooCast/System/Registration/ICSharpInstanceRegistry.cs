using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface ICSharpInstanceRegistry<KeyType, ValueType> : IRegistry<KeyType, ValueType> where KeyType : ICSharpInstanceIdentifier where ValueType : ICSharpInstanceIdentifiable
    {
        
    }
}
