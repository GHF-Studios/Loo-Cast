using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    public abstract class MetaData : IMetaData
    {
        #region Properties
        public abstract HierarchyElement HierarchyElement { get; }
        public abstract IIdentifier Identifier { get; }
        public abstract IMetaData ParentMetaData { get; }
        public abstract IEnumerable<IMetaData> ChildMetaData { get; }
        #endregion

        #region Methods
        public abstract bool Validate();
        #endregion
    }
}
