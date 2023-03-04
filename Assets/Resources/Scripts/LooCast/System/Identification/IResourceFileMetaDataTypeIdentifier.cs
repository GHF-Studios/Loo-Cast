using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceFileMetaDataTypeIdentifier : IResourceObjectMetaDataTypeIdentifier
    {
        #region Properties
        string ParentResourceFileMetaDataTypeID { get; }
        string ResourceFileMetaDataTypeID { get; }
        #endregion
    }
}