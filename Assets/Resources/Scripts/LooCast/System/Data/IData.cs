using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Identification;

    public interface IData : IObject, IDataIdentifiable
    {
        #region Properties
        public string ResourcePath { get; }
        public IData? ParentData { get; }
        public SerializableList<IData> ChildData { get; }
        public DataKind DataKind { get; }
        #endregion
    }
}
