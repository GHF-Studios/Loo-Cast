using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface ICSharpInstanceTypeIdentifier : IInstanceTypeIdentifier
    {
        #region Properties
        string ParentCSharpInstanceTypeID { get; }
        string CSharpInstanceTypeID { get; }
        #endregion
    }
}