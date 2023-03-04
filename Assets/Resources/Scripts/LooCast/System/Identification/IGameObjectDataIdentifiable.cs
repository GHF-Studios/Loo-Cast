using System;

namespace LooCast.System.Identification
{
    public interface IGameObjectDataIdentifiable : IUnityInstanceDataIdentifiable
    {
        #region Properties
        IGameObjectDataIdentifier GameObjectDataIdentifier { get; }
        #endregion
    }
}
