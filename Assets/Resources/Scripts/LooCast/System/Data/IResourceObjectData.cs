using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IResourceObjectData : IResourceData, IResourceObjectDataIdentifiable
    {
        #region Properties
        public IResourceObjectDataType ResourceObjectDataType { get; }
        public IResourceObjectData? ParentResourceObjectData { get; }
        public SerializableList<IResourceObjectData> ChildResourceObjectData { get; }
        #endregion
    }
}
