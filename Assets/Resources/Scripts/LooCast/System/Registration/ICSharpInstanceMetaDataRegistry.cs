using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface ICSharpInstanceMetaDataRegistry<KeyType, ValueType> : IInstanceMetaDataRegistry<KeyType, ValueType> where KeyType : ICSharpInstanceMetaDataIdentifier where ValueType : ICSharpInstanceMetaDataIdentifiable
    {
        
    }
}
