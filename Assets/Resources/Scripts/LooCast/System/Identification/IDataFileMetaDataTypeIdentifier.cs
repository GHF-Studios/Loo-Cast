using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IDataFileMetaDataTypeIdentifier : IDataObjectMetaDataTypeIdentifier
    {
        #region Properties
        string ParentDataFileMetaDataTypeID { get; }
        string DataFileMetaDataTypeID { get; }
        #endregion
    }
}