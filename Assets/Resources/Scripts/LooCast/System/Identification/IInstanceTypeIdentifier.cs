using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IInstanceTypeIdentifier : ITypeIdentifier
    {
        #region Properties
        string ParentInstanceTypeID { get; }
        string InstanceTypeID { get; }
        #endregion
    }
}