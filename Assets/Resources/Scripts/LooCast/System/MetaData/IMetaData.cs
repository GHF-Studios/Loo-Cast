namespace LooCast.System.MetaData
{
    using LooCast.System.Collections.Generic;

    public interface IMetaData : IObject, IMetaDataIdentifiable
    {
        #region Properties
        public string Name { get; }
        public string Description { get; }
        public string Author { get; }
        public string Version { get; }
        public IMetaData? ParentMetaData { get; }
        public SerializableList<IMetaData> ChildMetaData { get; }
        public IDataIdentifier DataIdentifier { get; set; }
        #endregion
    }
}
