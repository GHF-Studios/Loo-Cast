namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface IObjectMetaData : ICSharpInstanceMetaData, IObjectMetaDataIdentifiable
    {
        #region Properties
        public IObjectMetaData? ParentObjectMetaData { get; }
        public SerializableList<IObjectMetaData> ChildObjectMetaData { get; }
        public IObjectDataIdentifier ObjectDataIdentifier { get; set; }
        #endregion
    }
}
