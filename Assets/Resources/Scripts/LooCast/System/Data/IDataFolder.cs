using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IDataFolder : IDataObject, IDataFolderIdentifiable
    {
        #region Properties
        public string ResourceFolderPath { get; }
        public IDataFolder ParentDataFolder { get; }
        public SerializableList<IDataFolder> ChildDataFolders { get; }
        public SerializableList<IDataFile> ChildDataFiles { get; }
        #endregion

        #region Methods
        public IResourceFolder Serialize();
        #endregion
    }
}
