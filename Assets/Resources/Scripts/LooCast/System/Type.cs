using CSSystem = System;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;
    
    public class Type : IGenericIdentifiable<Type>
    {
        #region Properties
        public TypeIdentifier TypeIdentifier => typeIdentifier;
        public IIdentifier Identifier => typeIdentifier;
        public Type ParentType => parentType;
        public List<Type> ChildTypes => childTypes;
        #endregion

        #region Fields
        private TypeIdentifier typeIdentifier;
        private Type parentType;
        private List<Type> childTypes;
        #endregion

        #region Constructors
        internal Type(CSSystem.Type systemType, Namespace rootNamespace)
        {
            typeIdentifier = new TypeIdentifier((NamespaceIdentifier)rootNamespace.Identifier, systemType);
            parentType = null;
            childTypes = new List<Type>();
        }
        
        internal Type(CSSystem.Type systemType, Namespace rootNamespace, Type parentType)
        {
            typeIdentifier = new TypeIdentifier((NamespaceIdentifier)rootNamespace.Identifier, systemType);
            this.parentType = parentType;
            childTypes = new List<Type>();
        }
        #endregion

        #region Methods
        internal void AddChildType(Type childType)
        {
            childTypes.Add(childType);
        }
        #endregion
    }
}
