using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface ITypeMetaData : IMetaData
    {
        #region Properties
        public TypeIdentifier TypeIdentifier { get; }
        public ITypeMetaData ParentTypeMetaData { get; }
        public IEnumerable<ITypeMetaData> ChildTypesMetaData { get; }
        #endregion
    }
}
