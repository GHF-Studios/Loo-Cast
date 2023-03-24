using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class DataFolderRegistry : Registry<IDataFolderIdentifier, IDataFolderIdentifiable>, IDataFolderRegistry<IDataFolderIdentifier, IDataFolderIdentifiable>
    {
        public DataFolderRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
