namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface IResourceMetaData : IObject, IResourceMetaDataIdentifiable
    {
        #region Properties
        public IResourceMetaData? ParentResourceMetaData { get; }
        public SerializableList<IResourceMetaData> ChildResourceMetaData { get; }
        public IResourceDataIdentifier ResourceDataIdentifier { get; set; }
        #endregion
    }
}
