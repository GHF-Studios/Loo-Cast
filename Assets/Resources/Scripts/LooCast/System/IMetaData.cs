﻿using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaData : IHierarchyElement
    {
        #region Properties
        public IMetaData ParentMetaData { get; }
        public IEnumerable<IMetaData> ChildMetaData { get; }
        #endregion

        #region Methods
        public bool Validate();
        #endregion
    }
}
