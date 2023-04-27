using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface IInstanceTypeMetaData : ITypeMetaData
    {
        #region Properties
        public IInstanceTypeMetaData ParentInstanceTypeMetaData { get; }
        
        public IEnumerable<IInstanceTypeMetaData> ChildInstanceTypesMetaData { get; }
        #endregion
    }
}
