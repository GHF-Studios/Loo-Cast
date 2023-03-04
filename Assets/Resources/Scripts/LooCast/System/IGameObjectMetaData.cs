namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface IGameObjectMetaData : IUnityInstanceMetaData, IGameObjectMetaDataIdentifiable
    {
        #region Properties
        public IGameObjectMetaData? ParentGameObjectMetaData { get; }
        public SerializableList<IGameObjectMetaData> ChildGameObjectMetaData { get; }
        public IGameObjectDataIdentifier GameObjectDataIdentifier { get; set; }
        #endregion
    }
}
