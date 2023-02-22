using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IGameObjectTypeIdentifier : IUnityInstanceTypeIdentifier
    {
        #region Properties
        string ParentGameObjectTypeID { get; }
        string GameObjectTypeID { get; }
        #endregion
    }
}