using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceFolderDataTypeIdentifier : IResourceObjectDataTypeIdentifier
    {
        #region Properties
        string ParentResourceFolderDataTypeID { get; }
        string ResourceFolderDataTypeID { get; }
        #endregion
    }
}