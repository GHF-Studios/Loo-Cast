using CSSystem = System;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;
    
    public class Type : IType
    {
        #region Properties
        public IIdentifier Identifier => typeIdentifier;
        public ITypeIdentifier TypeIdentifier => typeIdentifier;
        public INamespace TypeNamespace => typeNamespace;
        public CSSystem.Type CSSystemType => typeIdentifier.CSSystemType;
        public IType ParentType => parentType;
        public List<IType> ChildTypes => childTypes;
        #endregion

        #region Fields
        private ITypeIdentifier typeIdentifier;
        private INamespace typeNamespace;
        private IType parentType;
        private List<IType> childTypes;
        #endregion

        #region Constructors
        public Type(CSSystem.Type cssystemType, INamespace rootNamespace)
        {
            typeIdentifier = new TypeIdentifier((NamespaceIdentifier)rootNamespace.Identifier, cssystemType);
            parentType = null;
            childTypes = new List<IType>();
        }

        public Type(CSSystem.Type systemType, INamespace rootNamespace, IType parentType)
        {
            typeIdentifier = new TypeIdentifier((NamespaceIdentifier)rootNamespace.Identifier, systemType);
            this.parentType = parentType;
            childTypes = new List<IType>();
        }
        #endregion

        #region Methods
        public void AddChildType(Type childType)
        {
            childTypes.Add(childType);
        }
        #endregion
    }
}
