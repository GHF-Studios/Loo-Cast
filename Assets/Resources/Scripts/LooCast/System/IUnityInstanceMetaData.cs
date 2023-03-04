namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface IUnityInstanceMetaData : IInstanceMetaData, IUnityInstanceMetaDataIdentifiable
    {
        #region Properties
        public IUnityInstanceMetaData? ParentUnityInstanceMetaData { get; }
        public SerializableList<IUnityInstanceMetaData> ChildUnityInstanceMetaData { get; }
        public IUnityInstanceDataIdentifier UnityInstanceDataIdentifier { get; set; }
        #endregion
    }
}
