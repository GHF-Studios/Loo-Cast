namespace LooCast.System.MetaData
{
    using LooCast.System.Collections.Generic;

    public interface IResourceObjectMetaData : IResourceMetaData, IResourceObjectMetaDataIdentifiable
    {
        #region Properties
        public IResourceObjectMetaData? ParentResourceObjectMetaData { get; }
        public SerializableList<IResourceObjectMetaData> ChildResourceObjectMetaData { get; }
        public IResourceObjectDataIdentifier ResourceObjectDataIdentifier { get; set; }
        #endregion
    }
}
