using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    public abstract class TypeMetaData : MetaData, ITypeMetaData
    {
        #region Properties
        public abstract TypeIdentifier TypeIdentifier { get; }
        public abstract ITypeMetaData ParentTypeMetaData { get; }
        public abstract IEnumerable<ITypeMetaData> ChildTypesMetaData { get; }
        #endregion
    }
}
