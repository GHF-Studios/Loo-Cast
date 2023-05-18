using System;

namespace LooCast.System
{
    public interface IIdentifiableObject
    {
        #region Properties
        IObjectIdentifier ObjectIdentifier { get; }
        #endregion
    }
}
