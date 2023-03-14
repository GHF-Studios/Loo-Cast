using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Collections.Generic;
    [Serializable]
    public abstract class Data : Object, IData
    {
        #region Properties
        public IDataType DataType { get; }
        public IData? ParentData { get; }
        public SerializableList<IData> ChildData { get; }
        #endregion

        #region Constructors
        public Data(IDataType dataType, IData? parentData = null)
        {
            DataType = dataType;
            ParentData = parentData;
            ChildData = new SerializableList<IData>();
        }
        #endregion

        #region Methods
        public string SerializeRecursively()
        {
            throw null;
        }

        public void DeserializeRecursively(string serializedData)
        {
            throw null;
        }

        public void AddChildData(IData childData)
        {
            ChildData.Add(childData);
        }

        public void RemoveChildData(IData childData)
        {
            ChildData.Remove(childData);
        }
        #endregion
    }
}
