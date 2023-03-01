using System;

namespace LooCast.System.Identification
{
    public interface IDataIdentifiable : IObjectIdentifiable
    {
        #region Properties
        IDataIdentifier DataIdentifier { get; }
        #endregion
    }
}
