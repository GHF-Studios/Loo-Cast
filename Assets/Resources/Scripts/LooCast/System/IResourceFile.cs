using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IResourceFile : IResourceObject, IPersistable, IResourceFileIdentifiable
    {
        #region Properties
        public string ResourceFilePath { get; }
        public IResourceFileType ResourceFileType { get; }
        public IResourceFolder? ParentResourceFolder { get; }
        public SerializableList<IResourceObject> ChildResourceObjects { get; }
        #endregion
    }
}
