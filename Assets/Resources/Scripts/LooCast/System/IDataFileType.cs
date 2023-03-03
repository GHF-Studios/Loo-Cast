using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IDataFileType : IDataObjectType
    {
        #region Properties
        public IDataFileType ParentDataFileType { get; }
        public List<IDataFileType> ChildDataFileTypes { get; }
        #endregion
    }
}
