using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IResourceFolderData : IResourceObjectData, IResourceFolderDataIdentifiable
    {
        #region Properties
        public IResourceFolderDataType ResourceFolderDataType { get; }
        public IResourceFolderData? ParentResourceFolderData { get; }
        public SerializableList<IResourceFolderData> ChildResourceFolderData { get; }
        #endregion
    }
}
