using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IData : IObject, IDataIdentifiable
    {
        #region Properties
        public IDataType DataType { get; }
        public IData? ParentData { get; }
        public SerializableList<IData> ChildData { get; }
        #endregion

        #region Methods
        public string SerializeRecursively();
        public void DeserializeRecursively(string serializedData);
        #endregion
    }
}
