using System;
using System.Collections.Generic;

namespace LooCast.Identifier
{
    public interface IIdentifiableType : IIdentifiable
    {
        #region Properties
        Type Type { get; }
        string TypeName { get; }
        IIdentifiableType ParentType { get; }
        List<IIdentifiableType> ChildTypes { get; }
        IIdentifiableNamespace TypeNamespace { get; }
        #endregion

        #region Methods
        void AddChildType(IIdentifiableType childType);
        void AddChildTypes(IEnumerable<IIdentifiableType> childTypes);
        #endregion
    }
}