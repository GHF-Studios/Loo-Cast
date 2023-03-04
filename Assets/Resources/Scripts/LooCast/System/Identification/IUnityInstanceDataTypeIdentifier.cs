using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IUnityInstanceDataTypeIdentifier : IInstanceDataTypeIdentifier
    {
        #region Properties
        string ParentUnityInstanceDataTypeID { get; }
        string UnityInstanceDataTypeID { get; }
        #endregion
    }
}