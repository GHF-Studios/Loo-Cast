using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class TypeData : Data, ITypeData
    {
        #region Properties
        public abstract ITypeData TypeDataParent { get; }
        public abstract IEnumerable<ITypeData> TypeDataChildren { get; }

        public abstract IType ParentType { get; }
        public abstract IEnumerable<IType> ChildTypes { get; }
        #endregion
    }
}
