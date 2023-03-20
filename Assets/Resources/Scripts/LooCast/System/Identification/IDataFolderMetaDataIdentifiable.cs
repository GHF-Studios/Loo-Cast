using System;

namespace LooCast.System.Identification
{
    public interface IDataFolderMetaDataIdentifiable : IDataObjectMetaDataIdentifiable
    {
        #region Properties
        IDataFolderMetaDataIdentifier DataFolderMetaDataIdentifier { get; }
        #endregion
    }
}
