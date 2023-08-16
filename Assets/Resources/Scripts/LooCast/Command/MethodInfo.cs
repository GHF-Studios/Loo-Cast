using System;

namespace LooCast.Command
{
    using LooCast.System;
    
    public sealed class MethodInfo
    {
        #region Classes
        public sealed class Implementation
        {
            #region Properties
            public MethodInfo Method { get; }
            public ParameterInfo[] Parameters { get; }
            public string MethodSignature { get; }
            public string FullMethodSignature { get; }
            public global::System.Reflection.MethodInfo SystemMethodInfo { get; }
            public MethodDocumentation Documentation { get; }
            #endregion

            #region Constructors
            public Implementation(MethodInfo method, ParameterInfo[] parameters, global::System.Reflection.MethodInfo systemMethodInfo, MethodDocumentation documentation)
            {
                if (method == null)
                {
                    throw new ArgumentNullException(nameof(method));
                }
                if (parameters == null)
                {
                    throw new ArgumentNullException(nameof(parameters));
                }
                if (systemMethodInfo == null)
                {
                    throw new ArgumentNullException(nameof(systemMethodInfo));
                }
                if (documentation == null)
                {
                    throw new ArgumentNullException(nameof(documentation));
                }
                foreach (ParameterInfo parameter in parameters)
                {
                    if (parameter == null)
                    {
                        throw new ArgumentException("Parameter cannot be null!", nameof(parameters));
                    }
                }

                Method = method;
                Parameters = parameters;

                string[] parameterIdentifiers = new string[parameters.Length];
                string[] fullParameterIdentifiers = new string[parameters.Length];

                for (int j = 0; j < parameters.Length; j++)
                {
                    ParameterInfo parameter = parameters[j];
                    parameterIdentifiers[j] = $"{parameter.ParameterType.TypeName} {parameter.ParameterName}";
                    fullParameterIdentifiers[j] = $"{parameter.ParameterType.FullTypeName} {parameter.ParameterName}";
                }

                if (method.ReturnType != null)
                {
                    MethodSignature = $"{method.ReturnType.TypeName} {method.MethodName}({string.Join(", ", parameterIdentifiers)})";
                    FullMethodSignature = $"{method.ReturnType.FullTypeName} {method.MethodName}({string.Join(", ", fullParameterIdentifiers)})";
                }
                else
                {
                    MethodSignature = $"void {method.MethodName}({string.Join(", ", parameterIdentifiers)})";
                    FullMethodSignature = $"void {method.MethodName}({string.Join(", ", fullParameterIdentifiers)})";
                }

                SystemMethodInfo = systemMethodInfo;
                Documentation = documentation;
            }
            #endregion
        }
        #endregion

        #region Properties
        public string MethodName { get; }
        public NamespaceInfo ParentNamespace { get; }
        public TypeInfo ReturnType { get; }
        private Implementation[] Implementations { get; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public MethodInfo(string methodName, NamespaceInfo parentNamespace, TypeInfo returnType, Implementation[] implementations)
        {
            if (methodName == null)
            {
                throw new ArgumentNullException(nameof(methodName));
            }
            if (parentNamespace == null)
            {
                throw new ArgumentNullException(nameof(parentNamespace));
            }
            if (implementations == null)
            {
                throw new ArgumentNullException(nameof(implementations));
            }
            if (!StringUtil.IsAlphaNumeric(methodName))
            {
                throw new ArgumentException($"Method name '{methodName}' is not alphanumeric!");
            }
            if (implementations.Length == 0)
            {
                throw new ArgumentException("Method must have at least one implementation!");
            }

            MethodName = methodName;
            ParentNamespace = parentNamespace;
            ReturnType = returnType;
            Implementations = implementations;
        }
        #endregion

        #region Methods
        public void InvokeAction(params object[] arguments)
        {
            
        }

        public object InvokeFunction(params object[] arguments)
        {
        }
        #endregion
    }
}
