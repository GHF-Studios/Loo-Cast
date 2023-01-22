using LooCast.Util.Collections.Generic;
using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    public class NamespaceIdentifier : IIdentifiableNamespace
    {
        #region Properties
        public string ID
        {
            get
            {
                return $"{FullName}";
            }
        }
        public string FullName
        {
            get
            {
                if (ParentNamespace == null)
                {
                    return Name;
                }
                else
                {
                    return $"{ParentNamespace.Name}.{Name}";
                }
            }
        }
        
        public string Name => name;
        public IIdentifiableNamespace ParentNamespace => parentNamespace;
        public List<IIdentifiableNamespace> ChildNamespaces => childNamespaces.Values;
        #endregion

        #region Fields
        [SerializeField] private string name;
        [SerializeField] private IIdentifiableNamespace parentNamespace;
        [SerializeField] private SerializableList<IIdentifiableNamespace> childNamespaces;
        #endregion

        #region Constructors
        internal NamespaceIdentifier(string name)
        {
            this.name = name;
            parentNamespace = null;
            childNamespaces = null;
        }

        internal NamespaceIdentifier(string name, IIdentifiableNamespace parentNamespace)
        {
            this.name = name;
            this.parentNamespace = parentNamespace;
            childNamespaces = null;
        }
        #endregion

        #region Methods
        public void AddChildNamespace(IIdentifiableNamespace childNamespace)
        {
            if (childNamespaces == null)
            {
                childNamespaces = new SerializableList<IIdentifiableNamespace>();
            }
            
            if (childNamespaces.Contains(childNamespace))
            {
                throw new Exception($"Namespace '{childNamespace.Name}' already exists in {Name}!");
            }

            childNamespaces.Add(childNamespace);
        }

        public void AddChildNamespaces(IEnumerable<IIdentifiableNamespace> childNamespaces)
        {
            if (this.childNamespaces == null)
            {
                this.childNamespaces = new SerializableList<IIdentifiableNamespace>();
            }

            foreach (IIdentifiableNamespace childNamespace in childNamespaces)
            {
                if (this.childNamespaces.Contains(childNamespace))
                {
                    throw new Exception($"Namespace '{childNamespace.Name}' already exists in {Name}!");
                }
            }

            this.childNamespaces.AddRange(childNamespaces);
        }
        #endregion
    }
}
