using System;

namespace LooCast.System
{
    public interface INamespace
    {
        #region Properties
        public IIdentifier Identifier => NamespaceIdentifier;
        public NamespaceIdentifier NamespaceIdentifier { get; }
        public string NamespaceName => NamespaceIdentifier.NamespaceName;

        public Namespace ParentNamespace { get; }
        public NamespaceRegistry ChildNamespaces { get; } = new NamespaceRegistry();
        public TypeRegistry ContainedTypes { get; } = new TypeRegistry();
        #endregion
    }
}
