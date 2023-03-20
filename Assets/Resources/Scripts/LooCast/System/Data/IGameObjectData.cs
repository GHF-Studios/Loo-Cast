using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IGameObjectData : IUnityInstanceData, IGameObjectDataIdentifiable
    {
        #region Properties
        public IGameObjectDataType GameObjectDataType { get; }
        public IGameObjectData? ParentGameObjectData { get; }
        public SerializableList<IGameObjectData> ChildGameObjectData { get; }
        #endregion
    }
}
