namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    public class Namespace : ILooCastObject
    {
        #region Properties
        public IIdentifier Identifier => NamespaceIdentifier;
        public NamespaceIdentifier NamespaceIdentifier { get; }
        public string NamespaceName => NamespaceIdentifier.NamespaceName;

        public Namespace ParentNamespace { get; }
        public NamespaceRegistry ChildNamespaces { get; } = new NamespaceRegistry();
        public TypeRegistry ContainedTypes { get; } = new TypeRegistry();
        #endregion

        #region Constructors
        public Namespace(NamespaceIdentifier namespaceIdentifier, Namespace parentNamespace = null)
        {
            NamespaceIdentifier = namespaceIdentifier;
            ParentNamespace = parentNamespace;

            parentNamespace?.ChildNamespaces.Add(namespaceIdentifier, this);
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is Namespace otherNamespace)
            {
                return Equals(otherNamespace);
            }
            return false;
        }

        public bool Equals(Namespace otherNamespace)
        {
            return NamespaceIdentifier.Equals(otherNamespace.NamespaceIdentifier);
        }

        public override int GetHashCode()
        {
            return NamespaceIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return NamespaceIdentifier.ToString();
        }
        #endregion
    }
}
