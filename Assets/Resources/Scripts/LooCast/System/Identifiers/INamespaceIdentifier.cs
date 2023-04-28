using System;

namespace LooCast.System.Identifiers
{
    public interface INamespaceIdentifier : IIdentifier
    {
        #region Properties
        public string NamespaceName { get; }
#nullable enable
        public INamespaceIdentifier? ParentNamespaceIdentifier { get; }
#nullable disable
        #endregion
    }
}
