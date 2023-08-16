using System;

namespace LooCast.Command
{
    using LooCast.System;
    
    public sealed class TypeInfo
    {
        #region Properties
        public string TypeName { get; }
        public TypeVariant TypeVariant { get; }
        public NamespaceInfo ParentNamespace { get; }
        public Type SystemType { get; }
        public string FullTypeName { get; }
        public TypeDocumentation Documentation { get; }
        #endregion

        #region Constructors
        public TypeInfo(string typeName, TypeVariant typeVariant, NamespaceInfo parentNamespace, Type systemType, TypeDocumentation documentation)
        {
            if (typeName == null)
            {
                throw new ArgumentNullException(nameof(typeName));
            }
            if (parentNamespace == null)
            {
                throw new ArgumentNullException(nameof(parentNamespace));
            }
            if (systemType == null)
            {
                throw new ArgumentNullException(nameof(systemType));
            }
            if (documentation == null)
            {
                throw new ArgumentNullException(nameof(documentation));
            }
            if (!StringUtil.IsAlphaNumeric(typeName))
            {
                throw new ArgumentException($"Type name '{typeName}' is not alphanumeric!");
            }

            TypeName = typeName;
            TypeVariant = typeVariant;
            ParentNamespace = parentNamespace;
            SystemType = systemType;
            FullTypeName = $"{parentNamespace.FullNamespaceName}.{typeName}";
            Documentation = documentation;
        }
        #endregion

        #region Methods
        public bool IsAssignableFrom(TypeInfo type)
        {
            return IsAssignableFrom(type.SystemType);
        }
        
        public bool IsAssignableFrom(Type systemType)
        {
            return SystemType.IsAssignableFrom(systemType);
        }
        #endregion
    }
}
