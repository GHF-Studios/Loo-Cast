using LooCast.Core;
using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    public class IdentifierManager
    {
        #region Static Properties
        public static IdentifierManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new IdentifierManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static IdentifierManager instance;
        #endregion

        #region Properties
        public Dictionary<string, IIdentifiableNamespace> RootNamespaces => rootNamespaces;
        public Dictionary<string, IIdentifiableType> RootTypes => rootTypes;
        public Dictionary<string, IIdentifiableInstance> RootInstances => rootInstances;
        #endregion

        #region Fields
        private Dictionary<string, IIdentifiableNamespace> rootNamespaces;
        private Dictionary<string, IIdentifiableType> rootTypes;
        private Dictionary<string, IIdentifiableInstance> rootInstances;
        #endregion

        internal void Initialize()
        {
            rootNamespaces = new Dictionary<string, IIdentifiableNamespace>();
            rootTypes = new Dictionary<string, IIdentifiableType>();
            rootInstances = new Dictionary<string, IIdentifiableInstance>();
        }

        public void CreateRootNamespace(string namespaceName)
        {
            if (string.IsNullOrEmpty(namespaceName))
            {
                throw new Exception($"[IdentifierManager] Namespace name cannot be empty!");
            }
            if (rootNamespaces.ContainsKey(namespaceName))
            {
                throw new Exception($"[IdentifierManager] Root Namespace '{namespaceName}' already exists!");
            }
            NamespaceIdentifier newRootNamespace = new NamespaceIdentifier(namespaceName);
            rootNamespaces.Add(newRootNamespace.ID, newRootNamespace);
        }
        
        public void CreateNamespace(IIdentifiableNamespace parentNamespace, string namespaceName)
        {
            NamespaceIdentifier newChildNamespace = new NamespaceIdentifier(namespaceName, parentNamespace);
            if (string.IsNullOrEmpty(namespaceName))
            {
                throw new Exception($"[IdentifierManager] Namespace name cannot be empty!");
            }
            if (parentNamespace.ChildNamespaces.Contains(newChildNamespace))
            {
                throw new Exception($"[IdentifierManager] Namespace '{namespaceName}' in parent '{parentNamespace.ID}' already exists!");
            }
            parentNamespace.AddChildNamespace(newChildNamespace);
        }

        public void CreateRootType(IIdentifiableNamespace parentNamespace, Type type)
        {
            if (rootTypes.ContainsKey(type.Name))
            {
                throw new Exception($"[IdentifierManager] Root Type '{type.Name}' already exists!");
            }
            TypeIdentifier newRootType = new TypeIdentifier(parentNamespace, type);
            rootTypes.Add(newRootType.ID, newRootType);
        }

        public void CreateType(IIdentifiableType parentType, Type type)
        {
            TypeIdentifier newChildType = new TypeIdentifier(parentType, type);
            if (parentType.ChildTypes.Contains(newChildType))
            {
                throw new Exception($"[IdentifierManager] Type '{type.Name}' in parent '{parentType.ID}' already exists!");
            }
            parentType.AddChildType(newChildType);
        }

        public void CreateRootInstance(IIdentifiableType parentType, ExtendedMonoBehaviour objectInstance)
        {
            InstanceIdentifier newRootInstance = new InstanceIdentifier(parentType, objectInstance.InstanceID);
            if (rootInstances.ContainsKey(newRootInstance.ID))
            {
                throw new Exception($"[IdentifierManager] Root Instance '{newRootInstance.ID}' already exists!");
            }
            rootInstances.Add(newRootInstance.ID, newRootInstance);
        }

        public void CreateInstance(IIdentifiableType parentType, ExtendedMonoBehaviour instance, IIdentifiableInstance parentInstance)
        {
            InstanceIdentifier newChildInstance = new InstanceIdentifier(parentType, instance.InstanceID, parentInstance);
            if (parentInstance.ChildInstances.Contains(newChildInstance))
            {
                throw new Exception($"[IdentifierManager] Instance '{newChildInstance.ID}' in parent '{parentInstance.ID}' already exists!");
            }
            parentInstance.AddChildInstance(newChildInstance);
        }
    } 
}