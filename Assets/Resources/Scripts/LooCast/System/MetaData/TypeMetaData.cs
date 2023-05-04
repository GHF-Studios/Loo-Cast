using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    public abstract class TypeMetaData : MetaData, ITypeMetaData
    {
        #region Properties
        public abstract ITypeIdentifier TypeIdentifier { get; }
        public abstract ITypeMetaData TypeMetaDataParent { get; }
        public abstract IEnumerable<ITypeMetaData> TypeMetaDataChildren { get; }

        public abstract IType TypeParent { get; }
        public abstract IEnumerable<IType> TypeChildren { get; }
        #endregion
    }
}
