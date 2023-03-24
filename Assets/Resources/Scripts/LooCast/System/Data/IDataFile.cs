using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Identification;
    using LooCast.System.Resources;

    public interface IDataFile : IData, IDataFileIdentifiable
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
