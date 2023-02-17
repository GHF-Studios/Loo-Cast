using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IUnityInstanceTypeIdentifier : IInstanceTypeIdentifier
    {
        #region Properties
        string ParentUnityInstanceTypeID { get; }
        string UnityInstanceTypeID { get; }
        #endregion
    }
}