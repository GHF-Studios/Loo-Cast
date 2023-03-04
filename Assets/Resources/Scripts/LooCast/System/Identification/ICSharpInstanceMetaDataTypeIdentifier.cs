using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface ICSharpInstanceMetaDataTypeIdentifier : IInstanceMetaDataTypeIdentifier
    {
        #region Properties
        string ParentCSharpInstanceMetaDataTypeID { get; }
        string CSharpInstanceMetaDataTypeID { get; }
        #endregion
    }
}