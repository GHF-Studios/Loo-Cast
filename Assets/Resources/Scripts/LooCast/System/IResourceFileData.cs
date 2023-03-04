using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IResourceFileData : IResourceObjectData, IResourceFileDataIdentifiable
    {
        #region Properties
        public IResourceFileDataType ResourceFileDataType { get; }
        public IResourceFileData? ParentResourceFileData { get; }
        public SerializableList<IResourceFileData> ChildResourceFileData { get; }
        #endregion
    }
}
