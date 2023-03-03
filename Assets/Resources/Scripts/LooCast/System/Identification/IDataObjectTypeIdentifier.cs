using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IDataObjectTypeIdentifier : IDataTypeIdentifier
    {
        #region Properties
        string ParentDataObjectTypeID { get; }
        string DataObjectTypeID { get; }
        #endregion
    }
}