using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IDataFolderTypeIdentifier : IDataObjectTypeIdentifier
    {
        #region Properties
        string ParentDataFolderTypeID { get; }
        string DataFolderTypeID { get; }
        #endregion
    }
}