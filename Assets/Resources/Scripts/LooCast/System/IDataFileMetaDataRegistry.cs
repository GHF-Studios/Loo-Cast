using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IDataFileMetaDataRegistry<KeyType, ValueType> : IDataObjectMetaDataRegistry<KeyType, ValueType> where KeyType : IDataFileMetaDataIdentifier where ValueType : IDataFileMetaDataIdentifiable
    {
        
    }
}
