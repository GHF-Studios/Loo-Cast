using System;

namespace LooCast.Command
{
    using LooCast.System;

    [AttributeUsage(AttributeTargets.Class | AttributeTargets.Struct, Inherited = false, AllowMultiple = false)]
    public sealed class TypeAttribute : Attribute
    {
        #region Properties
        public string TypeName { get; }
        public TypeVariant TypeVariant { get; }
        public TypeDocumentation TypeDocumentation { get; }
        #endregion

        #region Constructors
        public TypeAttribute(string typeName, TypeVariant typeVariant, TypeDocumentation documentation)
        {
            if (StringUtil.IsEmpty(typeName))
            {
                throw new ArgumentNullException(nameof(typeName));
            }
            if (documentation == null)
            {
                throw new ArgumentNullException(nameof(documentation));
            }

            TypeName = typeName;
            TypeVariant = typeVariant;
        }
        #endregion
    }
}
