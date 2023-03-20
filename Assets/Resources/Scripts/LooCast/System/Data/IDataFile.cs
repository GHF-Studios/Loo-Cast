using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IDataFile : IDataObject, IDataFileIdentifiable, IPersistable
    {
        #region Properties
        public IDataFileType DataFileType { get; }
        public IDataFolder ParentDataFolder { get; }
        public SerializableList<IDataObject> ChildDataObjects { get; }
        public string DataFilePath { get; }
        #endregion
    }
}
