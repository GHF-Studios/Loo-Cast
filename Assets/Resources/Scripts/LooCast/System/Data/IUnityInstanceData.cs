using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IUnityInstanceData : IInstanceData, IUnityInstanceDataIdentifiable
    {
        #region Properties
        public IUnityInstanceDataType UnityInstanceDataType { get; }
        public IUnityInstanceData? ParentUnityInstanceData { get; }
        public SerializableList<IUnityInstanceData> ChildUnityInstanceData { get; }
        #endregion
    }
}
