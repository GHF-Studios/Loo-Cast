using System;

namespace LooCast.System.Identification
{
    public interface IDataFileIdentifiable : IDataObjectIdentifiable
    {
        #region Properties
        IDataFileIdentifier DataFileIdentifier { get; }
        #endregion
    }
}
