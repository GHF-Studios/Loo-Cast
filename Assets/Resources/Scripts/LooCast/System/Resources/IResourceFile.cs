using System.Collections.Generic;

namespace LooCast.System.Resources
{
    using LooCast.System.Identification;

    public interface IResourceFile : IResourceObject, IResourceFileIdentifiable
    {
        #region Properties
        public string ResourceFilePath { get; }
        public IResourceFolder? ParentResourceFolder { get; }
        public SerializableList<IResourceObject> ChildResourceObjects { get; }
        #endregion

        #region Methods
        public IDataFile Deserialize();
        #endregion
    }
}
