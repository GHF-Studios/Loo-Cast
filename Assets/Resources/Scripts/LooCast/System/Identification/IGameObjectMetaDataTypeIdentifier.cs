using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IGameObjectMetaDataTypeIdentifier : IUnityInstanceMetaDataTypeIdentifier
    {
        #region Properties
        string ParentGameObjectMetaDataTypeID { get; }
        string GameObjectMetaDataTypeID { get; }
        #endregion
    }
}