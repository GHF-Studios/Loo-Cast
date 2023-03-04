using System;

namespace LooCast.System.Identification
{
    public interface IUnityInstanceMetaDataIdentifiable : IInstanceMetaDataIdentifiable
    {
        #region Properties
        IUnityInstanceMetaDataIdentifiable UnityInstanceMetaDataIdentifier { get; }
        #endregion
    }
}
