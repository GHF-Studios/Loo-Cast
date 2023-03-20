using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IDataFolderMetaDataRegistry<KeyType, ValueType> : IDataObjectMetaDataRegistry<KeyType, ValueType> where KeyType : IDataFolderMetaDataIdentifier where ValueType : IDataFolderMetaDataIdentifiable
    {
        
    }
}
