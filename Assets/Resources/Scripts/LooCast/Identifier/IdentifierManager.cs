using System;
using System.Collections.Generic;

namespace LooCast.Identifier
{
    public class IdentifierManager
    {
        #region Fields
        private Dictionary<string, NamespaceIdentifier> rootNamespaces;
        #endregion

        internal void Initialize()
        {
            rootNamespaces = new Dictionary<string, NamespaceIdentifier>();
        }

        public void CreateRootNamespace(string namespaceName)
        {
            rootNamespaces.Add(namespaceName, new NamespaceIdentifier(namespaceName));
        }

        public NamespaceIdentifier GetRootNamespace(string namespaceName)
        {
            if (!rootNamespaces.ContainsKey(namespaceName))
            {
                throw new Exception($"[IdentifierManager] Root Namespace '{namespaceName}' does not exist!");
            }
            rootNamespaces.TryGetValue(namespaceName, out NamespaceIdentifier rootNamespace);
            return rootNamespace;
        }
    } 
}