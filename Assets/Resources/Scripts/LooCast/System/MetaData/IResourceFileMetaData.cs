namespace LooCast.System.MetaData
{
    using LooCast.System.Collections.Generic;

    public interface IResourceFileMetaData : IResourceObjectMetaData, IResourceFileMetaDataIdentifiable
    {
        #region Properties
        public IResourceFileMetaData? ParentResourceFileMetaData { get; }
        public SerializableList<IResourceFileMetaData> ChildResourceFileMetaData { get; }
        public IResourceFileDataIdentifier ResourceFileDataIdentifier { get; set; }
        #endregion
    }
}
