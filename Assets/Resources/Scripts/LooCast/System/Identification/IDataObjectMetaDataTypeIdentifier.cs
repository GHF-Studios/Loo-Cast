using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IDataObjectMetaDataTypeIdentifier : IMetaDataTypeIdentifier
    {
        #region Properties
        string ParentDataObjectMetaDataTypeID { get; }
        string DataObjectMetaDataTypeID { get; }
        #endregion
    }
}