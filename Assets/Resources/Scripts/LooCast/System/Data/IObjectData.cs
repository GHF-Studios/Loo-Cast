using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IObjectData : ICSharpInstanceData, IObjectDataIdentifiable
    {
        #region Properties
        public IObjectDataType ObjectDataType { get; }
        public IObjectData? ParentObjectData { get; }
        public SerializableList<IObjectData> ChildObjectData { get; }
        #endregion
    }
}
