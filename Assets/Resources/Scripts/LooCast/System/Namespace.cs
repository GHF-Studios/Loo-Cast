using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;
    
    public class Namespace : INamespace
    {
        #region Properties
        public INamespaceIdentifier NamespaceIdentifier => namespaceIdentifier;
        public IIdentifier Identifier => namespaceIdentifier;
        public Namespace ParentNamespace => parentNamespace;
        public List<Namespace> ChildNamespaces => childNamespaces;
        #endregion

        #region Fields
        private NamespaceIdentifier namespaceIdentifier;
        private Namespace parentNamespace;
        private List<Namespace> childNamespaces;
        #endregion

        #region Constructors
        internal Namespace(string name)
        {
            namespaceIdentifier = new NamespaceIdentifier(name);
            parentNamespace = null;
            childNamespaces = new List<Namespace>();
        }

        internal Namespace(string name, Namespace parentNamespace)
        {
            NamespaceIdentifier parentNamespaceIdentifier = (NamespaceIdentifier)parentNamespace.Identifier;
            namespaceIdentifier = new NamespaceIdentifier(name, parentNamespaceIdentifier.NamespaceID);
            this.parentNamespace = parentNamespace;
            childNamespaces = new List<Namespace>();
        }

        internal Namespace(NamespaceIdentifier namespaceIdentifier)
        {
            this.namespaceIdentifier = namespaceIdentifier;
            parentNamespace = null;
            childNamespaces = new List<Namespace>();
        }
        #endregion

        #region Methods
        internal void AddChildNamespace(Namespace childNamespace)
        {
            childNamespaces.Add(childNamespace);
        }
        #endregion
    }
}
