namespace LooCast.System.MetaData
{
    using LooCast.System.Collections.Generic;

    public interface IComponentMetaData : IGameObjectMetaData, IComponentMetaDataIdentifiable
    {
        #region Properties
        public IComponentMetaData? ParentComponentMetaData { get; }
        public SerializableList<IComponentMetaData> ChildComponentMetaData { get; }
        public IComponentDataIdentifier ComponentDataIdentifier { get; set; }
        #endregion
    }
}
