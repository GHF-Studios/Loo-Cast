using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public class Namespace
    {
        #region Properties
        public NamespaceIdentifier Identifier => new NamespaceIdentifier(namespaceName, parentNamespace.Identifier);
        public Namespace ParentNamespace => parentNamespace;
        public string NamespaceName => namespaceName;
        public List<Namespace> ChildNamespaces => childNamespaces;
        public List<Type> ContainedTypes => containedTypes;
        #endregion

        #region Fields
        private Namespace parentNamespace;
        private string namespaceName;
        private List<Namespace> childNamespaces;
        private List<Type> containedTypes;
        #endregion

        #region Constructors
        public Namespace(string namespaceName, Namespace parentNamespace = null)
        {
            this.namespaceName = namespaceName;
            this.parentNamespace = parentNamespace;
        }
        #endregion

        #region Methods
        public void RegisterChildNamespace(Namespace childNamespace)
        {
            if (childNamespaces.Contains(childNamespace))
            {
                throw new global::System.Exception($"Child Namespace '{childNamespace}' is already registered!");
            }
            childNamespaces.Add(childNamespace);
        }

        public void RegisterContainedType(Type containedType)
        {
            if (containedTypes.Contains(containedType))
            {
                throw new global::System.Exception($"Contained Type '{containedType}' is already registered!");
            }
            containedTypes.Add(containedType);
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
