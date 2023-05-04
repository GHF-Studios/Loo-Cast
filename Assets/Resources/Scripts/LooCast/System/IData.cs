using System.Collections.Generic;

namespace LooCast.System
{
    public interface IData : IIdentifiable, IHierarchyElement
    {
        #region Properties
        public IData DataParent { get; }
        public IEnumerable<IData> DataChildren { get; }
        #endregion
    }
}
