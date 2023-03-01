using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IDataFolder : IDataObject, IDataFolderIdentifiable, IPersistable
    {
        #region Properties
        public IDataFolderType DataFolderType { get; }
        public IDataFolder ParentDataFolder { get; }
        public SerializableList<IDataFolder> ChildDataFolders { get; }
        public SerializableList<IDataFile> ChildDataFiles { get; }
        public string DataFolderPath { get; }
        #endregion
    }
}
