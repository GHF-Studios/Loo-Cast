using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;

namespace LooCast.System.CSharp
{
    public sealed class MethodInfo
    {
        #region Properties
        public global::System.Reflection.MethodInfo Method { get; }

        public IEnumerable<Attribute> AllAttributes { get; }
        public IEnumerable<Attribute> DirectAttributes { get; }
        public IEnumerable<Attribute> InheritedAttributes { get; }

        public bool IsPublic { get; }
        public bool IsStatic { get; }
        public bool IsInstance { get; }
        public bool IsImplemented { get; }
        #endregion

        #region Constructors
        public MethodInfo(global::System.Reflection.MethodInfo method)
        {
            Method = method;

            AllAttributes = (Attribute[])method.GetCustomAttributes(true);
            DirectAttributes = (Attribute[])method.GetCustomAttributes(false);
            InheritedAttributes = AllAttributes.Except(DirectAttributes);

            IsPublic = method.IsPublic;
            IsStatic = method.IsStatic;
            IsInstance = !IsStatic;
            IsImplemented = !method.IsAbstract;
        }
        #endregion
    }
}
