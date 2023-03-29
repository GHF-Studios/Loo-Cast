

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Registration;

    public class Type
    {
        #region Properties
        public TypeIdentifier Identifier
        {
            get
            {
                if (identifier == null)
                {
                    identifier = new TypeIdentifier(TypeName, ContainingNamespace.Identifier);
                }
                return identifier.Value;
            }
        }
        
        public string TypeName => typeName;
        
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
        private TypeIdentifier? identifier;
#nullable disable

        private string typeName;
        
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
        public Type(string typeName, Namespace containingNamespace, Type parentType = null)
        {
            this.typeName = typeName;
            
            this.containingNamespace = containingNamespace;

            this.parentType = parentType;
            childTypes = new TypeRegistry();
            
            containedGameObjects = new GameObjectRegistry();
            containedComponents = new ComponentRegistry();
            containedSystemObjects = new SystemObjectRegistry();
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
            return Identifier.Equals(otherType.Identifier);
        }

        public override int GetHashCode()
        {
            return Identifier.GetHashCode();
        }

        public override string ToString()
        {
            return Identifier.ToString();
        }
        #endregion
    }
}
