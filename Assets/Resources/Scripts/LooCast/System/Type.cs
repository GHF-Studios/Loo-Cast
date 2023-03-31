using CSSystem = System;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Managers;
    using LooCast.System.Registries;

    public class Type : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => typeIdentifier;
        public TypeIdentifier TypeIdentifier => typeIdentifier;

        public string FullTypeName => fullTypeName;
        public CSSystem.Type CSSystemType => cssystemType;
#nullable enable
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
        public Type(global::System.Type cssystemType, Type? parentType = null)
        {
            TypeIdentifier.TryParse(cssystemType, out typeIdentifier!);

            fullTypeName = typeIdentifier.FullTypeName;
            this.cssystemType = cssystemType;

            Type[] genericTypeArguments = new Type[typeIdentifier.GenericTypeArgumentIdentifiers!.Length];
            TypeManager typeManager = TypeManager.Instance;
            for (int i = 0; i < genericTypeArguments.Length; i++)
            {
                genericTypeArguments[i] = typeManager.GetType(typeIdentifier.GenericTypeArgumentIdentifiers[i]);
            }

            NamespaceIdentifier.TryParse(cssystemType.Namespace, out NamespaceIdentifier containingNamespaceIdentifier);
            containingNamespace = NamespaceManager.Instance.GetNamespace(containingNamespaceIdentifier);

            if (parentType != null)
            {
                parentType.ChildTypes.Add(typeIdentifier, this);
            }
            this.parentType = parentType;
            childTypes = new TypeRegistry();
            
            containedGameObjects = new GameObjectRegistry();
            containedComponents = new ComponentRegistry();
            containedSystemObjects = new SystemObjectRegistry();
        }
#nullable disable
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
