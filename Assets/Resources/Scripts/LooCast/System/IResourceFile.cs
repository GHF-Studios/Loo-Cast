using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IResourceFile : IResourceObject, IResourceFileIdentifiable, IPersistable
    {
        #region Properties
        public IResourceFileType ResourceFileType { get; }
        public IResourceFolder ParentResourceFolder { get; }
        public SerializableList<IResourceObject> ChildResourceObjects { get; }
        public string ResourceFilePath { get; }
        #endregion
    }
}
