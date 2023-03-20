using CSSystem = System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    using System.Identification;
    
    public interface IType : ITypeIdentifiable
    {
        #region Properties
        public INamespace TypeNamespace { get; }
        public CSSystem.Type CSSystemType { get; }
        public IType ParentType { get; }
        public List<IType> ChildTypes { get; }
        #endregion
    }
}