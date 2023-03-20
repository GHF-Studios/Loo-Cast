using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    
    public interface IResourceFolderRegistry<KeyType, ValueType> : IResourceObjectRegistry<KeyType, ValueType> where KeyType : IResourceFolderIdentifier where ValueType : IResourceFolderIdentifiable
    {
        
    }
}
