using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IObjectTypeIdentifier : IInstanceTypeIdentifier
    {
        #region Properties
        string ParentObjectTypeID { get; }
        string ObjectTypeID { get; }
        #endregion
    }
}