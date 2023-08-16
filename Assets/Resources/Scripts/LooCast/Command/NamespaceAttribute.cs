using System;

namespace LooCast.Command
{
    using LooCast.System;
    
    [AttributeUsage(AttributeTargets.Class, Inherited = false, AllowMultiple = false)]
    public sealed class NamespaceAttribute : Attribute
    {
        #region Properties
        public string NamespaceName { get; }
        public NamespaceVariant NamespaceVariant { get; }
        public NamespaceDocumentation NamespaceDocumentation { get; }
        #endregion

        #region Constructors
        public NamespaceAttribute(string namespaceName, NamespaceVariant namespaceVariant, NamespaceDocumentation namespaceDocumentation)
        {
            if (StringUtil.IsEmpty(namespaceName))
            {
                throw new ArgumentNullException(nameof(namespaceName));
            }
            if (namespaceDocumentation == null)
            {
                throw new ArgumentNullException(nameof(namespaceDocumentation));
            }

            NamespaceName = namespaceName;
            NamespaceVariant = namespaceVariant;
            NamespaceDocumentation = namespaceDocumentation;
        }
        #endregion
    }
}
