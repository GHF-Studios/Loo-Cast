using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;
    using LooCast.System.Management;

    public class Namespace : INamespace
    {
        #region Properties
        public IIdentifier Identifier => namespaceIdentifier;
        public INamespaceIdentifier NamespaceIdentifier => namespaceIdentifier;
        public string NamespaceName => namespaceName;
        public INamespace ParentNamespace => parentNamespace;
        public List<INamespace> ChildNamespaces => childNamespaces;
        #endregion

        #region Fields
        protected INamespaceIdentifier namespaceIdentifier;
        protected string namespaceName;
        protected INamespace parentNamespace;
        protected List<INamespace> childNamespaces;
        #endregion

        #region Constructors
        public Namespace(string namespaceName)
        {
            namespaceIdentifier = new NamespaceIdentifier(namespaceName);
            this.namespaceName = namespaceName;
            parentNamespace = null;
            childNamespaces = new List<INamespace>();
        }

        public Namespace(string namespaceName, INamespace parentNamespace)
        {
            NamespaceIdentifier parentNamespaceIdentifier = (NamespaceIdentifier)parentNamespace.Identifier;
            namespaceIdentifier = new NamespaceIdentifier(namespaceName, parentNamespaceIdentifier.NamespaceID);
            this.namespaceName = namespaceName;
            this.parentNamespace = parentNamespace;
            childNamespaces = new List<INamespace>();
        }
        #endregion

        #region Methods
        public void AddChildNamespace(INamespace childNamespace)
        {
            childNamespaces.Add(childNamespace);
        }
        #endregion
    }
}
