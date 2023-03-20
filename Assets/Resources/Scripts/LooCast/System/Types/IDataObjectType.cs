using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IDataObjectType : IDataType
    {
        #region Properties
        public IDataObjectType ParentDataObjectType { get; }
        public List<IDataObjectType> ChildDataObjectTypes { get; }
        #endregion
    }
}
