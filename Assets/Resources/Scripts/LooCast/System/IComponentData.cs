using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IComponentData : IGameObjectData, IComponentDataIdentifiable
    {
        #region Properties
        public IComponentDataType ComponentDataType { get; }
        public IComponentData? ParentComponentData { get; }
        public SerializableList<IComponentData> ChildComponentData { get; }
        #endregion
    }
}
