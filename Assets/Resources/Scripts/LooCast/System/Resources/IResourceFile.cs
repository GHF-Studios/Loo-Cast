using System.Collections.Generic;

namespace LooCast.System.Resources
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Data;
    using LooCast.System.Identification;

    public interface IResourceFile : IResource, IResourceFileIdentifiable
    {
        #region Properties
        public string ResourceFilePath { get; }
        public global::System.IO.FileInfo ResourceFileInfo { get; }
        public IResourceFolder? ParentResourceFolder { get; }
        public SerializableList<IResourceObject> ChildResourceObjects { get; }
        #endregion

        #region Methods
        public IDataFile Deserialize();
        #endregion
    }
}
