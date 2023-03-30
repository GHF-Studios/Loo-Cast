

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    public class Namespace
    {
        #region Properties
        public NamespaceIdentifier Identifier => identifier;

        public string NamespaceName => namespaceName;

#nullable enable
        public Namespace? ParentNamespace => parentNamespace;
#nullable disable
        public NamespaceRegistry ChildNamespaces => childNamespaces;
        
        public TypeRegistry ContainedTypes => containedTypes;
        #endregion

        #region Fields
#nullable enable
        private NamespaceIdentifier? identifier;
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
            identifier = new NamespaceIdentifier(namespaceName, parentNamespace?.Identifier);
            
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
            return Identifier.Equals(otherNamespace.Identifier);
        }

        public override int GetHashCode()
        {
            return Identifier.GetHashCode();
        }

        public override string ToString()
        {
            return Identifier.ToString();
        }
        #endregion
    }
}
