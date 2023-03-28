

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
        
        public Type? ParentType => parentType;
        public TypeRegistry ChildTypes => childTypes;
        
        public GameObjectRegistry ContainedGameObjects => containedGameObjects;
        public SystemObjectRegistry ContainedSystemObjects => containedSystemObjects;
        #endregion

        #region Fields
        private TypeIdentifier? identifier;
        
        private string typeName;
        
        private Namespace containingNamespace;
        
        private Type? parentType;
        private TypeRegistry childTypes;
        
        private GameObjectRegistry containedGameObjects;
        private SystemObjectRegistry containedSystemObjects;
        #endregion

        #region Constructors
        public Type(string typeName, Namespace containingNamespace)
        {
            this.typeName = typeName;
            
            this.containingNamespace = containingNamespace;
            
            parentType = null;
            childTypes = new TypeRegistry();
            
            containedGameObjects = new GameObjectRegistry();
            containedSystemObjects = new SystemObjectRegistry();
        }
         
        public Type(string typeName, Type parentType)
        {
            this.typeName = typeName;
            
            containingNamespace = parentType.ContainingNamespace;
            
            this.parentType = parentType;
            childTypes = new TypeRegistry();
            
            containedGameObjects = new GameObjectRegistry();
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
