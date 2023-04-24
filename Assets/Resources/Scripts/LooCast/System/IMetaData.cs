using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaData
    {
        #region Properties
#nullable enable
        public IMetaData? ParentMetaData { get; }
#nullable disable
        public HashSet<IMetaData> ChildMetaData { get; }
        #endregion

        #region Methods
        public bool Validate();
        #endregion
    }
}
