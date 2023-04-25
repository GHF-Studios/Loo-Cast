using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    
    public interface IType : ILooCastObject
    {
        #region Interfaces
        public interface IInstance : ILooCastObject
        {
            #region Properties
            public IType Type { get; }
            #endregion
        }
        #endregion

        #region Properties
        public TypeIdentifier TypeIdentifier { get; }
        public string FullTypeName => TypeIdentifier.FullTypeName;
        public Type CSSystemType { get; }

        public Namespace ContainingNamespace { get; }

        public IType ParentType { get; }
        public HashSet<IType> ChildTypes { get; }
        #endregion
    }
}
