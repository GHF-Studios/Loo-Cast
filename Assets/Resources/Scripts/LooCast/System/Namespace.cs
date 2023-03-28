

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Registration;

    public class Namespace
    {
        #region Properties
        public NamespaceIdentifier Identifier
        {
            get
            {
                if (identifier == null)
                {
                    identifier = new NamespaceIdentifier(NamespaceName, ParentNamespace?.Identifier);
                }
                return identifier.Value;
            }
        }
        
        public string NamespaceName => namespaceName;
        
        public Namespace ParentNamespace => parentNamespace;
        public NamespaceRegistry ChildNamespaces => childNamespaces;
        
        public TypeRegistry ContainedTypes => containedTypes;
        #endregion

        #region Fields
        private NamespaceIdentifier? identifier;
        
        private string namespaceName;
        
        private Namespace parentNamespace;
        private NamespaceRegistry childNamespaces;
        
        private TypeRegistry containedTypes;
        #endregion

        #region Constructors
        public Namespace(string namespaceName, Namespace parentNamespace = null)
        {
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
