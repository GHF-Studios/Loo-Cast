using System;

namespace LooCast.System.Identification
{
    public interface IResourceFolderIdentifiable : IResourceObjectIdentifiable
    {
        #region Properties
        IResourceFolderIdentifier ResourceFolderIdentifier { get; }
        #endregion
    }
}
