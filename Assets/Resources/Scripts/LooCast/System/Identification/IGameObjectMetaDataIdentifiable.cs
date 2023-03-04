using System;

namespace LooCast.System.Identification
{
    public interface IGameObjectMetaDataIdentifiable : IUnityInstanceMetaDataIdentifiable
    {
        #region Properties
        IGameObjectMetaDataIdentifiable GameObjectMetaDataIdentifier { get; }
        #endregion
    }
}
