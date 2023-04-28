using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface ITypeMetaData : IMetaData
    {
        #region Properties
        public ITypeIdentifier TypeIdentifier { get; }
        public ITypeMetaData ParentTypeMetaData { get; }
        public IEnumerable<ITypeMetaData> ChildTypesMetaData { get; }

        public IType ParentType { get; }
        public IEnumerable<IType> ChildTypes { get; }
        #endregion
    }
}
