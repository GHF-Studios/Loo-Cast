using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaData : ILooCastObject
    {
        #region Properties
        public IMetaData ParentMetaData { get; }
        public IEnumerable<IMetaData> ChildMetaData { get; }
        #endregion
    }
}
