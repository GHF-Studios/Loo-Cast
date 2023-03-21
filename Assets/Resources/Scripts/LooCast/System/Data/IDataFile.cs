using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IDataFile : IDataObject, IDataFileIdentifiable
    {
        #region Properties
        public string ResourceFilePath { get; }
        public IDataFolder ParentDataFolder { get; }
        public SerializableList<IDataObject> ChildDataObjects { get; }
        #endregion

        #region Methods
        public IResourceFile Serialize();
        #endregion
    }
}
