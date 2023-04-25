using CSSystem = System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Data;
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;
    using LooCast.System.Registries;

    public class Type<TInstance> : IType, ILooCastObject where TInstance : Type<TInstance>, new()
    {
        #region Classes
        public class Instance
        {
            #region Properties
            public Identifier InstanceIdentifier;
            public virtual InstanceData Data
            {
                get
                {
                    // Get Data from this Instance(, which is composed of contained Data, which also is composed of the data that is contained within itself, and so on) in a recursive fashion.
                    
                }

                set
                {
                    // Set Data, if the Instance Skeleton is created, aka if the object is essentially not null.
                }
            }
            public virtual InstanceMetaData MetaData
            {
                get
                {
                    // Get MetaData from this Instance in the same recursive fashion as Data, just that MetaData is a different Property of this Instance, representing not the Data of the ready to use Instance, but the Skeleton of this Instance.
                }

                set
                {
                    // Set MetaData on this Instance by setting the Meta of all child instances in a recursive fashion.
                }
            }
            #endregion
        }
        #endregion

        #region Properties
        public Identifier Identifier => TypeIdentifier;
        public TypeIdentifier TypeIdentifier { get; }
        public string FullTypeName => TypeIdentifier.FullTypeName;
        public CSSystem.Type CSSystemType { get; }

        public Namespace ContainingNamespace { get; }

        public IType ParentType { get; }
        public HashSet<IType> ChildTypes { get; } = new HashSet<IType>();
        public ObjectPool<TInstance> InstancePool { get; } = new ObjectPool<TInstance>();
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
