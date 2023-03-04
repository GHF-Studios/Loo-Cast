using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IResourceFolderDataRegistry<KeyType, ValueType> : IResourceObjectDataRegistry<KeyType, ValueType> where KeyType : IResourceFolderDataIdentifier where ValueType : IResourceFolderDataIdentifiable
    {
        
    }
}
