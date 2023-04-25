using CSSystem = System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Data;
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;
    using LooCast.System.Registries;

    public abstract class Type<TInstance> : IType 
        where TInstance : IType.IInstance, new()
    {
        #region Properties
        public IIdentifier Identifier => TypeIdentifier;
        public TypeIdentifier TypeIdentifier { get; }
        public string FullTypeName => TypeIdentifier.FullTypeName;
        public CSSystem.Type CSSystemType { get; }

        public Namespace ContainingNamespace { get; }

        public IType ParentType { get; }
        public HashSet<IType> ChildTypes { get; } = new HashSet<IType>();
        #endregion

        #region Constructors
        protected Type(CSSystem.Type cssystemType)
        {
            TypeIdentifier = TypeIdentifier.Parse(cssystemType);
            CSSystemType = cssystemType;
            // Get Parent CSSystem.Type, Parse it to a TypeIdentifier, get the Type from the TypeRegistry and assign it to the ParentType property.
            NamespaceRegistry namespaceRegistry = MainManager.Instance.MainRegistry.GetRegistry(typeof(Namespace)) as NamespaceRegistry;
            ContainingNamespace = namespaceRegistry.GetValue(cssystemType.Namespace);
            ParentType?.ChildTypes.Add(this);
        }
        #endregion

        #region Methods
        public bool IsSubtypeOf(IType otherType)
        {
            return CSSystemType.IsSubclassOf(otherType.CSSystemType);
        }

        public bool HasGenericTypeArgument(IType expectedGenericArgument)
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
