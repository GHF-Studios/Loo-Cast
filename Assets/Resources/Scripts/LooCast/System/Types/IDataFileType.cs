using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IDataFileType : IDataObjectType
    {
        #region Properties
        public IDataFileType ParentDataFileType { get; }
        public List<IDataFileType> ChildDataFileTypes { get; }
        #endregion
    }
}
