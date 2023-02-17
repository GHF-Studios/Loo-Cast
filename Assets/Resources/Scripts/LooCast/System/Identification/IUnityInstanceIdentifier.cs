using System;

namespace LooCast.System.Identification
{
    public interface IUnityInstanceIdentifier : IInstanceIdentifier
    {
        #region Properties
        string UnityInstanceTypeID { get; }
        string UnityInstanceGUID { get; }
        string UnityInstanceID { get; }
        #endregion    
    }
}
