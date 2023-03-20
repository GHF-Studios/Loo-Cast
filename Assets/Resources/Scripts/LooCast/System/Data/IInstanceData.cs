using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IInstanceData : IData, IInstanceDataIdentifiable
    {
        #region Properties
        public IInstanceDataType InstanceDataType { get; }
        public IInstanceData? ParentInstanceData { get; }
        public SerializableList<IInstanceData> ChildInstanceData { get; }
        #endregion
    }
}
