using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IDataFileTypeIdentifier : IDataObjectTypeIdentifier
    {
        #region Properties
        string ParentDataFileTypeID { get; }
        string DataFileTypeID { get; }
        #endregion
    }
}