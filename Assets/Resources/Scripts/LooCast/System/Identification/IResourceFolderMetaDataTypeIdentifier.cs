using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceFolderMetaDataTypeIdentifier : IResourceObjectMetaDataTypeIdentifier
    {
        #region Properties
        string ParentResourceFolderMetaDataTypeID { get; }
        string ResourceFolderMetaDataTypeID { get; }
        #endregion
    }
}