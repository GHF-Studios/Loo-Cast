namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface IResourceFolderMetaData : IResourceObjectMetaData, IResourceFolderMetaDataIdentifiable
    {
        #region Properties
        public IResourceFolderMetaData? ParentResourceFolderMetaData { get; }
        public SerializableList<IResourceFolderMetaData> ChildResourceFolderMetaData { get; }
        public IResourceFolderDataIdentifier ResourceFolderDataIdentifier { get; set; }
        #endregion
    }
}
