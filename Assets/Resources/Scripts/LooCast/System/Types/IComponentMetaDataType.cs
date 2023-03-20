using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IComponentMetaDataType : IGameObjectMetaDataType
    {
        #region Properties
        public IComponentMetaDataType ParentComponentMetaDataType { get; }
        public List<IComponentMetaDataType> ChildComponentMetaDataTypes { get; }
        #endregion
    }
}
