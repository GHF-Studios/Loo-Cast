namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface IInstanceMetaData : IData, IInstanceMetaDataIdentifiable
    {
        #region Properties
        public IInstanceMetaData? ParentInstanceMetaData { get; }
        public SerializableList<IInstanceMetaData> ChildInstanceMetaData { get; }
        public IInstanceDataIdentifier InstanceDataIdentifier { get; set; }
        #endregion
    }
}
