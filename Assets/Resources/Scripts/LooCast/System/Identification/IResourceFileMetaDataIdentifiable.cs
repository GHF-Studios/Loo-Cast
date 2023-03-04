using System;

namespace LooCast.System.Identification
{
    public interface IResourceFileMetaDataIdentifiable : IResourceObjectMetaDataIdentifiable
    {
        #region Properties
        IResourceFileMetaDataIdentifiable ResourceFileMetaDataIdentifier { get; }
        #endregion
    }
}
