using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Identification;
    using LooCast.System.Resources;

    public interface IDataFolder : IData, IDataFolderIdentifiable
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
