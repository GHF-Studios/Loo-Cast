using System;

namespace LooCast.System.Identifiers
{
    public interface ITypeIdentifier : IIdentifier
    {
        #region Properties
        public INamespaceIdentifier TypeNamespaceIdentifier { get;  }
        public string FullTypeName { get; }
        public Type CSSystemType { get; }
#nullable enable
        public ITypeIdentifier[]? GenericTypeArgumentIdentifiers { get; }
#nullable disable
        #endregion
    }
}
