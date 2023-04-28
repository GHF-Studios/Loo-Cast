using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class TypeData : Data, ITypeData
    {
        #region Properties
        public abstract ITypeData ParentTypeData { get; }
        public abstract IEnumerable<ITypeData> ChildTypeData { get; }

        public abstract IType ParentType { get; }
        public abstract IEnumerable<IType> ChildTypes { get; }
        #endregion
    }
}
