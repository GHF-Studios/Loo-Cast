using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IDataFolderMetaDataTypeIdentifier : IDataObjectMetaDataTypeIdentifier
    {
        #region Properties
        string ParentDataFolderMetaDataTypeID { get; }
        string DataFolderMetaDataTypeID { get; }
        #endregion
    }
}