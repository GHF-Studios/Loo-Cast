using CSSystem = System;

namespace LooCast.System
{
    using CSSystem;
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    public class Type : ILooCastObject
    {
        #region Classes
        public class TypeBuilder
        {
            public TypeIdentifier TypeIdentifier { get; private set; }
            public CSSystem.Type CSSystemType { get; private set; }
            public Type ParentType { get; private set; }

            public TypeBuilder WithCSSystemType(CSSystem.Type cssystemType)
            {
                CSSystemType = cssystemType;
                TypeIdentifier = TypeIdentifier.Parse(cssystemType);
                return this;
            }

            public TypeBuilder WithParentType(Type parentType)
            {
                ParentType = parentType;
                return this;
            }

            public virtual Type Build()
            {
                if (CSSystemType == null)
                {
                    throw new InvalidOperationException("CSSystemType must be provided.");
                }

                return new Type(this);
            }
        }
        #endregion
        
        #region Properties
        public Identifier Identifier => TypeIdentifier;
        public TypeIdentifier TypeIdentifier { get; }
        public string FullTypeName => TypeIdentifier.FullTypeName;
        public CSSystem.Type CSSystemType { get; }

        public Namespace ContainingNamespace { get; }

        public Type ParentType { get; }
        public TypeRegistry ChildTypes { get; } = new TypeRegistry();
        #endregion

        #region Constructors
        protected Type(TypeBuilder builder)
        {
            TypeIdentifier = builder.TypeIdentifier;
            CSSystemType = builder.CSSystemType;
            ParentType = builder.ParentType;
            NamespaceRegistry namespaceRegistry = MainManager.Instance.MainRegistry.GetRegistry(typeof(Namespace)) as NamespaceRegistry;
            ContainingNamespace = namespaceRegistry.GetValue(builder.CSSystemType.Namespace);
            ParentType?.ChildTypes.Add(TypeIdentifier, this);
        }
        #endregion

        #region Methods
        public bool IsSubtypeOf(Type otherType)
        {
            return CSSystemType.IsSubclassOf(otherType.CSSystemType);
        }

        public bool HasGenericTypeArgument(Type expectedGenericArgument)
        {
            if (!CSSystemType.IsGenericType)
            {
                return false;
            }

            foreach (CSSystem.Type genericArgument in CSSystemType.GetGenericArguments())
            {
                if (genericArgument == expectedGenericArgument.CSSystemType)
                {
                    return true;
                }
            }

            return false;
        }
        #endregion
    }
}
