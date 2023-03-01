using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IDataTypeIdentifier : IObjectTypeIdentifier
    {
        #region Properties
        string ParentDataTypeID { get; }
        string DataTypeID { get; }
        #endregion
    }
}