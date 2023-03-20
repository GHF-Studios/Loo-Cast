using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IResourceFolderMetaDataRegistry<KeyType, ValueType> : IResourceObjectMetaDataRegistry<KeyType, ValueType> where KeyType : IResourceFolderMetaDataIdentifier where ValueType : IResourceFolderMetaDataIdentifiable
    {
        
    }
}
