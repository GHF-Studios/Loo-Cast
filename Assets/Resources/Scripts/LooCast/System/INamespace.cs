using System;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;
    
    public interface INamespace : ILooCastObject
    {
        #region Properties
        public IIdentifier Identifier => NamespaceIdentifier;
        public NamespaceIdentifier NamespaceIdentifier { get; }
        public string NamespaceName => NamespaceIdentifier.NamespaceName;

        public Namespace ParentNamespace { get; }
        public NamespaceRegistry ChildNamespaces { get; }
        public TypeRegistry ContainedTypes { get; }
        #endregion
    }
}
