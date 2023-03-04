namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface IDataFileMetaData : IDataObjectMetaData, IDataFileMetaDataIdentifiable
    {
        #region Properties
        public IDataFileMetaData? ParentDataFileMetaData { get; }
        public SerializableList<IDataFileMetaData> ChildDataFileMetaData { get; }
        public IDataFileDataIdentifier DataFileIdentifier { get; set; }
        #endregion
    }
}
