using System;

namespace LooCast.System.Identification
{
    public interface IUnityInstanceDataIdentifiable : IInstanceDataIdentifiable
    {
        #region Properties
        IUnityInstanceDataIdentifier UnityInstanceDataIdentifier { get; }
        #endregion
    }
}
