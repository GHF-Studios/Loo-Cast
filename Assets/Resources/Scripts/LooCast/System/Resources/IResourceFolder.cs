using System.Collections.Generic;

namespace LooCast.System.Resources
{
    using LooCast.System.Identification;

    public interface IResourceFolder : IResourceObject, IPersistable, IResourceFolderIdentifiable
    {
        #region Properties
        public string ResourceFolderPath { get; }
        public IResourceFolderType ResourceFolderType { get; }
        public IResourceFolder? ParentResourceFolder { get; }
        public SerializableList<IResourceFolder> ChildResourceFolders { get; }
        public SerializableList<IResourceFile> ChildResourceFiles { get; }
        #endregion
    }
}
