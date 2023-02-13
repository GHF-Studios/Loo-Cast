using System;
using System.Collections.Generic;

namespace LooCast
{
    public class Namespace : IIdentifiable
    {
        #region Properties
        public IIdentifier Identifier => identifier;
        public Namespace ParentNamespace => parentNamespace;
        public List<Namespace> ChildNamespaces => childNamespaces;
        #endregion

        #region Fields
        private NamespaceIdentifier identifier;
        private Namespace parentNamespace;
        private List<Namespace> childNamespaces;
        #endregion

        #region Constructors
        internal Namespace(string name)
        {
            identifier = new NamespaceIdentifier(name);
            parentNamespace = null;
            childNamespaces = new List<Namespace>();
        }

        internal Namespace(string name, Namespace parentNamespace)
        {
            NamespaceIdentifier parentNamespaceIdentifier = (NamespaceIdentifier)parentNamespace.Identifier;
            identifier = new NamespaceIdentifier(name, parentNamespaceIdentifier.NamespaceID);
            this.parentNamespace = parentNamespace;
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
