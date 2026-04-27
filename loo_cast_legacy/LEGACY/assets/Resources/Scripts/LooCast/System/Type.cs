using CSSystem = System;

namespace LooCast.System
{
    using global::LooCast.System.Exceptions;
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.Registries;

    public class Type : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => typeIdentifier;
        public TypeIdentifier TypeIdentifier => typeIdentifier;

        public string FullTypeName => fullTypeName;
        public CSSystem.Type CSSystemType => cssystemType;
#nullable enable
        public Type[]? BaseTypes => baseTypes;
        public Type[]? GenericTypeArguments => genericTypeArguments;
#nullable disable

        public Namespace ContainingNamespace => containingNamespace;
        
#nullable enable
        public Type? ParentType => parentType;
#nullable disable
        public TypeRegistry ChildTypes => childTypes;
        
        public GameObjectRegistry ContainedGameObjects => containedGameObjects;
        public ComponentRegistry ContainedComponents => containedComponents;
        public SystemObjectRegistry ContainedSystemObjects => containedSystemObjects;
        #endregion

        #region Fields
#nullable enable
        private TypeIdentifier typeIdentifier;
#nullable disable

        private string fullTypeName;
        private CSSystem.Type cssystemType;
#nullable enable
        private Type[]? baseTypes;
        private Type[]? genericTypeArguments;
#nullable disable

        private Namespace containingNamespace;

#nullable enable
        private Type? parentType;
#nullable disable
        private TypeRegistry childTypes;
        
        private GameObjectRegistry containedGameObjects;
        private ComponentRegistry containedComponents;
        private SystemObjectRegistry containedSystemObjects;
        #endregion

        #region Constructors
#nullable enable
        public Type(global::System.Type cssystemType, Type[]? baseTypes = null, Type[]? genericTypeArguments = null, Type? parentType = null)
        {
            if(!TypeIdentifier.TryParse(cssystemType, out typeIdentifier!))
            {
                throw new InvalidTypeException($"[TypeManager] Type '{cssystemType}' is not a valid type");
            }

            fullTypeName = typeIdentifier.FullTypeName;


            NamespaceIdentifier.TryParse(cssystemType.Namespace, out NamespaceIdentifier? containingNamespaceIdentifier);
            containingNamespace = NamespaceManager.Instance.GetNamespace(containingNamespaceIdentifier);

            if (parentType != null)
            {
                parentType.ChildTypes.Add(typeIdentifier, this);
            }
            this.parentType = parentType;
            childTypes = new TypeRegistry();

            this.cssystemType = cssystemType;
            this.baseTypes = baseTypes;
            this.genericTypeArguments = genericTypeArguments;
            
            CheckBaseTypes(this, baseTypes);
            CheckGenericTypeArguments(this, genericTypeArguments);

            containedGameObjects = new GameObjectRegistry();
            containedComponents = new ComponentRegistry();
            containedSystemObjects = new SystemObjectRegistry();
        }
#nullable disable
        #endregion

        #region Methods
        public static void CheckBaseType(Type type, Type expectedBaseType)
        {
            if (!type.CSSystemType.IsAssignableFrom(expectedBaseType.CSSystemType))
            {
                throw new InvalidTypeException($"[TypeManager] Type '{type}' is not a subtype of expected basetype '{expectedBaseType}'!");
            }
        }

        public static void CheckBaseTypes(Type type, Type[] expectedBaseTypes)
        {
            foreach (Type expectedBaseType in expectedBaseTypes)
            {
                CheckBaseType(type, expectedBaseType);
            }
        }

        public static void CheckGenericTypeArgument(Type type, Type expectedGenericArgument)
        {
            if (!type.CSSystemType.IsGenericType || type.CSSystemType.GenericTypeArguments.Length != 1)
            {
                throw new InvalidTypeException($"[TypeManager] Type '{type}' does not have the expected generic type argument!");
            }

            if (!type.CSSystemType.GenericTypeArguments[0].IsAssignableFrom(expectedGenericArgument.CSSystemType))
            {
                throw new InvalidTypeException($"[TypeManager] Generic type argument '{type.GenericTypeArguments[0]}' is not assignable from the expected type '{expectedGenericArgument}'!");
            }
        }

        public static void CheckGenericTypeArguments(Type type, Type[] expectedGenericArguments)
        {
            if (!type.CSSystemType.IsGenericType || type.CSSystemType.GenericTypeArguments.Length != expectedGenericArguments.Length)
            {
                throw new InvalidTypeException($"[TypeManager] Type '{type}' does not have the expected generic type arguments!");
            }

            for (int i = 0; i < expectedGenericArguments.Length; i++)
            {
                if (!type.CSSystemType.GenericTypeArguments[i].IsAssignableFrom(expectedGenericArguments[i].CSSystemType))
                {
                    throw new InvalidTypeException($"[TypeManager] Generic type argument '{type.GenericTypeArguments[i]}' at index {i} is not assignable from the expected type '{expectedGenericArguments[i]}'!");
                }
            }
        }

        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is Type otherType)
            {
                return Equals(otherType);
            }
            return false;
        }

        public bool Equals(Type otherType)
        {
            return TypeIdentifier.Equals(otherType.TypeIdentifier);
        }

        public override int GetHashCode()
        {
            return TypeIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return TypeIdentifier.ToString();
        }
        #endregion
    }
}
