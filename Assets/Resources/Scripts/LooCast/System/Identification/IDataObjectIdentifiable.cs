using System;

namespace LooCast.System.Identification
{
    public interface IDataObjectIdentifiable : IDataIdentifiable
    {
        #region Properties
        IDataObjectIdentifier DataObjectIdentifier { get; }
        #endregion
    }
}
