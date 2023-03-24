using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Identification;
    using LooCast.System.Resources;

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
