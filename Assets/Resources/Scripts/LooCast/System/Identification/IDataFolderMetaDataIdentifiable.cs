using System;

namespace LooCast.System.Identification
{
    public interface IDataFileMetaDataIdentifiable : IDataObjectMetaDataIdentifiable
    {
        #region Properties
        IDataFileMetaDataIdentifier DataFileMetaDataIdentifier { get; }
        #endregion
    }
}
