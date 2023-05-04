using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaData : IIdentifiable, IHierarchyElement
    {
        #region Properties
        public IMetaData MetaDataParent { get; }
        public IEnumerable<IMetaData> MetaDataChildren { get; }

        public ILooCastObject Parent { get; }
        public IEnumerable<ILooCastObject> Children { get; }
        #endregion
    }
}
