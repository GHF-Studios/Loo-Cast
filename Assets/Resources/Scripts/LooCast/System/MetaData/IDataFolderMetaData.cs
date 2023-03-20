namespace LooCast.System.MetaData
{
    using LooCast.System.Collections.Generic;

    public interface IDataFolderMetaData : IDataObjectMetaData, IDataFolderMetaDataIdentifiable
    {
        #region Properties
        public IDataFolderMetaData? ParentDataFolderMetaData { get; }
        public SerializableList<IDataFolderMetaData> ChildDataFolderMetaData { get; }
        public IDataFolderDataIdentifier DataFolderIdentifier { get; set; }
        #endregion
    }
}
