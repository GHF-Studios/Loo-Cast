using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaData : ILooCastObject
    {
        #region Properties
        public IMetaData MetaDataParent { get; }
        public IEnumerable<IMetaData> MetaDataChildren { get; }
        #endregion
    }
}
