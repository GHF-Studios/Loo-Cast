using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IGameObjectDataTypeIdentifier : IUnityInstanceDataTypeIdentifier
    {
        #region Properties
        string ParentGameObjectDataTypeID { get; }
        string GameObjectDataTypeID { get; }
        #endregion
    }
}