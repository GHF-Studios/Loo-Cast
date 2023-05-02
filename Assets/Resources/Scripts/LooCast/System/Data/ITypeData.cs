using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;

    public interface ITypeData : IData
    {
        #region Properties
        public ITypeData ParentTypeData { get; }
        public IEnumerable<ITypeData> ChildTypeData { get; }

        public IType ParentType { get; }
        public IEnumerable<IType> ChildTypes { get; }
        #endregion
    }
}
