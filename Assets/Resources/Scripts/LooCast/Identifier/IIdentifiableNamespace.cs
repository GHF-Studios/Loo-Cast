using System.Collections.Generic;

namespace LooCast.Identifier
{
    public interface IIdentifiableNamespace : IIdentifiable
    {
        #region Properties
        string Name { get; }
        IIdentifiableNamespace ParentNamespace { get; }
        List<IIdentifiableNamespace> ChildNamespaces { get; }
        #endregion

        #region Methods
        void AddChildNamespace(IIdentifiableNamespace childNamespace);
        void AddChildNamespaces(IEnumerable<IIdentifiableNamespace> childNamespaces);
        #endregion
    }
}
