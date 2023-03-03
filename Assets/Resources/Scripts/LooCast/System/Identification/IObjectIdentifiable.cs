using System;

namespace LooCast.System.Identification
{
    public interface IObjectIdentifiable : ICSharpInstanceIdentifiable
    {
        #region Properties
        IObjectIdentifier ObjectIdentifier { get; }
        #endregion
    }
}
