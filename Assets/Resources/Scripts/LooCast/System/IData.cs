using System.Collections.Generic;

namespace LooCast.System
{
    public interface IData : ILooCastObject
    {
        #region Properties
        public IData DataParent { get; }
        public IEnumerable<IData> DataChildren { get; }
        #endregion
    }
}
