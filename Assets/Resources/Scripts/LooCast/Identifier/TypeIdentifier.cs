using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    [Serializable]
    public class TypeIdentifier : IIdentifiableType
    {
        public TypeIdentifier Parent { get; set; }
        public List<TypeIdentifier> Children { get; set; } = new List<TypeIdentifier>();
        public string TypeName
        {
            get
            {
                return Type.Name;
            }
        }
        public Type Type
        {
            get
            {
                return Type.GetType(TypeName);
            }
        }
        public string GUID
        {
            get
            {
                return TypeName;
            }
        }
        
        [SerializeField] private string typeAssemblyQualifiedName;

        public TypeIdentifier(Type type)
        {
            Name = GetType().Name;
            if (Parent != null)
            {
                GUID = Parent.GUID + "." + TypeName;
            }
            else
            {
                GUID = TypeName;
            }
        }
        public TypeIdentifier(TypeIdentifier parent)
        {
            Parent = parent;
            parent.Children.Add(this);
            Name = GetType().Name;
            GUID = Parent.GUID + "." + TypeName;
        }
    }
}