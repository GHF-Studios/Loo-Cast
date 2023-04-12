

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Registries;

    public class Namespace : ILooCastObject
    {
        #region Properties
        public Identifier Identifier => namespaceIdentifier;
        public NamespaceIdentifier NamespaceIdentifier => namespaceIdentifier;

        public string NamespaceName => namespaceName;

#nullable enable
        public Namespace? ParentNamespace => parentNamespace;
#nullable disable
        public NamespaceRegistry ChildNamespaces => childNamespaces;
        
        public TypeRegistry ContainedTypes => containedTypes;
        #endregion

        #region Fields
#nullable enable
        private NamespaceIdentifier? namespaceIdentifier;
#nullable disable
        
        private string namespaceName;

#nullable enable
        private Namespace? parentNamespace;
#nullable disable
        private NamespaceRegistry childNamespaces;
        
        private TypeRegistry containedTypes;
        #endregion

        #region Constructors
        public Namespace(string namespaceName, Namespace parentNamespace = null)
        {
            namespaceIdentifier = new NamespaceIdentifier(namespaceName, parentNamespace?.NamespaceIdentifier);
            
            this.namespaceName = namespaceName;
            
            this.parentNamespace = parentNamespace;
            childNamespaces = new NamespaceRegistry();
            
            containedTypes = new TypeRegistry();
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
