using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface IDataObject : IData, IDataObjectIdentifiable
    {
        #region Properties
        public string ResourceObjectPath { get; }
        public IDataObject? ParentDataObject { get; }
        public IDataFile? ParentDataFile { get; }
        public SerializableList<IDataObject> ChildDataObjects { get; }
        #endregion

        #region Methods
        public IResourceObject Serialize();
        #endregion
    }
}
