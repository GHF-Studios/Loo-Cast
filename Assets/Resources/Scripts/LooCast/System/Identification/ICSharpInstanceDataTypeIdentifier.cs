using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface ICSharpInstanceDataTypeIdentifier : IInstanceDataTypeIdentifier
    {
        #region Properties
        string ParentCSharpInstanceDataTypeID { get; }
        string CSharpInstanceDataTypeID { get; }
        #endregion
    }
}