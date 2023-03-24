using System.Collections.Generic;

namespace LooCast.System.Resources
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Data;
    using LooCast.System.Identification;

    public interface IResourceFolder : IResource, IResourceFolderIdentifiable
    {
        #region Properties
        public string ResourceFolderPath { get; }
        public global::System.IO.DirectoryInfo ResourceFolderInfo { get; }
        public IResourceFolder? ParentResourceFolder { get; }
        public SerializableList<IResourceFolder> ChildResourceFolders { get; }
        public SerializableList<IResourceFile> ChildResourceFiles { get; }
        #endregion

        #region Methods
        public IDataFolder Deserialize();
        #endregion
    }
}
