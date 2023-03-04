using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IObjectMetaDataTypeIdentifier : ICSharpInstanceMetaDataTypeIdentifier
    {
        #region Properties
        string ParentObjectMetaDataTypeID { get; }
        string ObjectMetaDataTypeID { get; }
        #endregion
    }
}