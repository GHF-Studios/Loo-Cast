using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IObjectDataTypeIdentifier : ICSharpInstanceDataTypeIdentifier
    {
        #region Properties
        string ParentObjectDataTypeID { get; }
        string ObjectDataTypeID { get; }
        #endregion
    }
}