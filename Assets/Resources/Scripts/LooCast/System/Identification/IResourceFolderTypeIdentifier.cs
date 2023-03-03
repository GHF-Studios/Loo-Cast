using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceFolderTypeIdentifier : IResourceObjectTypeIdentifier
    {
        #region Properties
        string ParentResourceFolderTypeID { get; }
        string ResourceFolderTypeID { get; }
        #endregion
    }
}