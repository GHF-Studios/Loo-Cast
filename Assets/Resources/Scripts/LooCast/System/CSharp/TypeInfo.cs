using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;

namespace LooCast.System.CSharp
{
    public sealed class TypeInfo
    {
        #region Properties
        public Type Type { get; }
        
        public IEnumerable<Attribute> AllAttributes { get; }
        public IEnumerable<Attribute> DirectAttributes { get; }
        public IEnumerable<Attribute> InheritedAttributes { get; }

        public MethodInfo[] Methods { get; }

        public bool IsGenericTypeDefinition { get; }
        public bool IsAbstract { get; }
        public bool IsPublic { get; }
        public bool IsNestedPublic { get; }
        public bool IsClass { get; }
        public bool IsValueType { get; }

        public string Namespace { get; }
        public string[] NamespaceParts { get; }
        #endregion

        #region Constructors
        public TypeInfo(Type type)
        {
            Type = type;

            AllAttributes = type.GetCustomAttributes(true).Cast<Attribute>();
            DirectAttributes = type.GetCustomAttributes(false).Cast<Attribute>();
            InheritedAttributes = AllAttributes.Except(DirectAttributes);
            
            Methods = type.GetMethods(BindingFlags.Public | BindingFlags.Instance | BindingFlags.Static | BindingFlags.DeclaredOnly).Select(method => new MethodInfo(method)).ToArray();

            IsGenericTypeDefinition = type.IsGenericTypeDefinition;
            IsAbstract = type.IsAbstract;
            IsPublic = type.IsPublic;
            IsNestedPublic = type.IsNestedPublic;
            IsClass = type.IsClass;
            IsValueType = type.IsValueType;

            Namespace = type.Namespace;
            if (Namespace != null)
            {
                NamespaceParts = Namespace.Split('.', StringSplitOptions.RemoveEmptyEntries);
            }
            else
            {
                NamespaceParts = Array.Empty<string>();
            }
        }
        #endregion
    }
}
