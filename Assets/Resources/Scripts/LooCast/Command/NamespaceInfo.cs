using System;
using System.Collections.Generic;

namespace LooCast.Command
{
    using LooCast.System;

    public sealed class NamespaceInfo
    {
        #region Properties
        public string NamespaceName { get; }
        public NamespaceVariant NamespaceVariant { get; }
        public NamespaceInfo ParentNamespace { get; }
        public string FullNamespaceName { get; }
        public NamespaceDocumentation Documentation { get; }
        #endregion

        #region Fields
        private Dictionary<string, TypeInfo> types;
        private Dictionary<string, MethodInfo> methods;
        private Dictionary<string, NamespaceInfo> childNamespaces;
        #endregion

        #region Constructors
        public NamespaceInfo(string namespaceName, NamespaceVariant namespaceVariant, NamespaceInfo parentNamespace, TypeInfo[] types, MethodInfo[] methods, NamespaceDocumentation documentation)
        {
            if (namespaceName == null)
            {
                throw new ArgumentNullException(nameof(namespaceName));
            }
            if (types == null)
            {
                throw new ArgumentNullException(nameof(types));
            }
            if (methods == null)
            {
                throw new ArgumentNullException(nameof(methods));
            }
            if (documentation == null)
            {
                throw new ArgumentNullException(nameof(documentation));
            }
            if (!StringUtil.IsAlphaNumeric(namespaceName))
            {
                throw new ArgumentException($"Namespace name '{namespaceName}' is not alphanumeric!");
            }

            NamespaceName = namespaceName;
            NamespaceVariant = namespaceVariant;
            ParentNamespace = parentNamespace;
            if (parentNamespace != null)
            {
                parentNamespace.AddChildNamespace(this);
                FullNamespaceName = $"{parentNamespace.FullNamespaceName}.{namespaceName}";

                switch (NamespaceVariant)
                {
                    case NamespaceVariant.CoreModule:
                        throw new ArgumentException($"Core module namespaces cannot have a parent namespace!");
                    case NamespaceVariant.Module:
                        if (parentNamespace.NamespaceVariant == NamespaceVariant.Module)
                        {
                            throw new ArgumentException($"Module namespaces cannot have a parent module namespace!");
                        }
                        else if (parentNamespace.NamespaceVariant == NamespaceVariant.SubModule)
                        {
                            throw new ArgumentException($"Module namespaces cannot have a parent submodule namespace!");
                        }
                        break;
                    case NamespaceVariant.SubModule:
                        if (parentNamespace.NamespaceVariant == NamespaceVariant.CoreModule)
                        {
                            throw new ArgumentException($"Submodule namespaces cannot have a parent core module namespace!");
                        }
                        break;
                }
            }
            else
            {
                FullNamespaceName = namespaceName;
            }
            this.types = new Dictionary<string, TypeInfo>();
            this.methods = new Dictionary<string, MethodInfo>();

            foreach (TypeInfo type in types)
            {
                this.types.Add(type.TypeName, type);
            }

            foreach (MethodInfo method in methods)
            {
                this.methods.Add(method.MethodName, method);
            }
            
            Documentation = documentation;
        }
        #endregion

        #region Methods
        public void AddChildNamespace(NamespaceInfo childNamespace)
        {
            if (childNamespace == null)
            {
                throw new ArgumentNullException(nameof(childNamespace));
            }
            if (childNamespace.ParentNamespace != this)
            {
                throw new ArgumentException($"Child namespace '{childNamespace.FullNamespaceName}' does not belong to namespace '{FullNamespaceName}'!");
            }
            if (childNamespaces.ContainsKey(childNamespace.NamespaceName))
            {
                throw new ArgumentException($"Child namespace '{childNamespace.FullNamespaceName}' already exists in namespace '{FullNamespaceName}'!");
            }

            childNamespaces.Add(childNamespace.NamespaceName, childNamespace);
        }

        public NamespaceInfo GetChildNamespace(string childNamespaceName)
        {
            if (!childNamespaces.TryGetValue(childNamespaceName, out NamespaceInfo childNamespace))
            {
                throw new ArgumentException($"Child namespace '{childNamespaceName}' not found in namespace '{FullNamespaceName}'!");
            }

            return childNamespace;
        }
        
        public TypeInfo GetType(string typeName)
        {
            if (!types.TryGetValue(typeName, out TypeInfo type))
            {
                throw new ArgumentException($"Type '{typeName}' not found in namespace '{FullNamespaceName}'!");
            }

            return type;
        }
        
        public MethodInfo GetMethod(string methodName)
        {
            if (!methods.TryGetValue(methodName, out MethodInfo method))
            {
                throw new ArgumentException($"Method '{methodName}' not found in namespace '{FullNamespaceName}'!");
            }

            return method;
        }

        public bool ContainsChildNamespace(string childNamespaceName)
        {
            return childNamespaces.ContainsKey(childNamespaceName);
        }

        public bool ContainsType(string typeName)
        {
            return types.ContainsKey(typeName);
        }

        public bool ContainsMethod(string methodName)
        {
            return methods.ContainsKey(methodName);
        }
        #endregion
    }
}
