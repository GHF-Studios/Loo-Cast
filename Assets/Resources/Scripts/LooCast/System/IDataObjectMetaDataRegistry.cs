using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IDataObjectMetaDataRegistry<KeyType, ValueType> : IMetaDataRegistry<KeyType, ValueType> where KeyType : IDataObjectMetaDataIdentifier where ValueType : IDataObjectMetaDataIdentifiable
    {
        
    }
}
