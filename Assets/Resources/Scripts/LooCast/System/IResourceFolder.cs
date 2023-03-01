using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IResourceFolder : IResourceObject, IResourceFolderIdentifiable, IPersistable
    {
        #region Properties
        public IResourceFolderType ResourceFolderType { get; }
        public IResourceFolder ParentResourceFolder { get; }
        public SerializableList<IResourceFolder> ChildResourceFolders { get; }
        public SerializableList<IResourceFile> ChildResourceFiles { get; }
        public string ResourceFolderPath { get; }
        #endregion
    }
}
