using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    using Util.Collections.Generic;
    
    [Serializable]
    public class TypeIdentifier : IIdentifiableType
    {
        #region Properties
        public string ID
        {
            get
            {
                return $"{TypeNamespace.ID}.{ParentType.ID}.{TypeName}";
            }
        }
        public Type Type
        {
            get
            {
                return Type.GetType(assemblyQualifiedTypeName);
            }
        }
        
        public string TypeName => typeName;
        public IIdentifiableType ParentType => parentType;
        public List<IIdentifiableType> ChildTypes => childTypes.Values;
        public IIdentifiableNamespace TypeNamespace => typeNamespace;
        #endregion

        #region Fields
        [SerializeField] private string typeName;
        [SerializeField] private string assemblyQualifiedTypeName;
        [SerializeField] private IIdentifiableType parentType;
        [SerializeField] private SerializableList<IIdentifiableType> childTypes;
        [SerializeField] private IIdentifiableNamespace typeNamespace;
        #endregion

        #region Constructors
        internal TypeIdentifier(IIdentifiableNamespace typeNamespace, Type type)
        {
            typeName = type.Name;
            assemblyQualifiedTypeName = type.AssemblyQualifiedName;
            parentType = null;
            childTypes = new SerializableList<IIdentifiableType>();
            this.typeNamespace = typeNamespace;
        }

        internal TypeIdentifier(IIdentifiableType parentType, Type type)
        {
            typeName = type.Name;
            assemblyQualifiedTypeName = type.AssemblyQualifiedName;
            this.parentType = parentType;
            childTypes = new SerializableList<IIdentifiableType>();
            typeNamespace = parentType.TypeNamespace;
        }
        #endregion

        #region Methods
        public void AddChildType(IIdentifiableType childType)
        {
            if (childTypes.Contains(childType))
            {
                throw new Exception($"[TypeIdentifier] Type '{childType.TypeName}' already exists in parent '{ID}'!");
            }
            childTypes.Add(childType);
        }

        public void AddChildTypes(IEnumerable<IIdentifiableType> childTypes)
        {
            if (this.childTypes == null)
            {
                this.childTypes = new SerializableList<IIdentifiableType>();
            }
            foreach (IIdentifiableType childType in childTypes)
            {
                if (this.childTypes.Contains(childType))
                {
                    throw new ArgumentException($"[TypeIdentifier] Type '{childType.ID}' already exists in parent '{ID}'!");
                }
                this.childTypes.Add(childType);
            }
        }
        #endregion
    }
}