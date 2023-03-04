using System;

namespace LooCast.System.Identification
{
    public interface IResourceFolderMetaDataIdentifiable : IResourceObjectMetaDataIdentifiable
    {
        #region Properties
        IResourceFolderMetaDataIdentifiable ResourceFolderMetaDataIdentifier { get; }
        #endregion
    }
}
