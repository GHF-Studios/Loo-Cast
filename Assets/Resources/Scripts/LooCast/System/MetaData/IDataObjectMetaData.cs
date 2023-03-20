namespace LooCast.System.MetaData
{
    using LooCast.System.Collections.Generic;

    public interface IDataObjectMetaData : IMetaData, IDataObjectMetaDataIdentifiable
    {
        #region Properties
        public IDataObjectMetaData? ParentDataObjectMetaData { get; }
        public SerializableList<IDataObjectMetaData> ChildDataObjectMetaData { get; }
        public IDataObjectDataIdentifier DataObjectIdentifier { get; set; }
        #endregion
    }
}
