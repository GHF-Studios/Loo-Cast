using System.Collections.Generic;

namespace LooCast.System
{
    public interface IData : ILooCastObject
    {
        #region Properties
        public IData ParentData { get; }
        public IEnumerable<IData> ChildData { get; }
        #endregion
    }
}
