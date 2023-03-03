using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IDataFolderRegistry<KeyType, ValueType> : IDataObjectRegistry<KeyType, ValueType> where KeyType : IDataFolderIdentifier where ValueType : IDataFolderIdentifiable
    {
        
    }
}
