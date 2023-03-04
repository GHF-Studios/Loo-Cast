using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IUnityInstanceMetaDataTypeIdentifier : IInstanceMetaDataTypeIdentifier
    {
        #region Properties
        string ParentUnityInstanceMetaDataTypeID { get; }
        string UnityInstanceMetaDataTypeID { get; }
        #endregion
    }
}