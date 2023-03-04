using System;

namespace LooCast.System.Identification
{
    public interface IResourceFolderDataIdentifiable : IResourceObjectDataIdentifiable
    {
        #region Properties
        IResourceFolderDataIdentifier ResourceFolderDataIdentifier { get; }
        #endregion
    }
}
