using System;

namespace LooCast.System.Identification
{
    public interface IDataObjectMetaDataIdentifiable : IMetaDataIdentifiable
    {
        #region Properties
        IDataObjectMetaDataIdentifier DataObjectMetaDataIdentifier { get; }
        #endregion
    }
}
