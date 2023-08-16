using System;

namespace LooCast.Command
{
    using LooCast.System;

    [AttributeUsage(AttributeTargets.Method, Inherited = false, AllowMultiple = false)]
    public sealed class MethodAttribute : Attribute
    {
        #region Properties
        public string MethodName { get; }
        public MethodDocumentation MethodDocumentation { get; }
        #endregion

        #region Constructors
        public MethodAttribute(string methodName, MethodDocumentation methodDocumentation)
        {
            if (StringUtil.IsEmpty(methodName))
            {
                throw new ArgumentNullException(nameof(methodName));
            }
            if (methodDocumentation == null)
            {
                throw new ArgumentNullException(nameof(methodDocumentation));
            }

            MethodName = methodName;
            MethodDocumentation = methodDocumentation;
        }
        #endregion
    }
}
