using System;

namespace LooCast.System.Identification
{
    public interface IObjectDataIdentifiable : ICSharpInstanceDataIdentifiable
    {
        #region Properties
        IObjectDataIdentifier ObjectDataIdentifier { get; }
        #endregion
    }
}
