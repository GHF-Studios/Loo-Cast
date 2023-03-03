using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IResourceData : IObjectData, IResourceDataIdentifiable
    {
        #region Properties
        public IResourceDataType ResourceDataType { get; }
        public IResourceData? ParentResourceData { get; }
        public SerializableList<IResourceData> ChildResourceData { get; }
        #endregion
    }
}
