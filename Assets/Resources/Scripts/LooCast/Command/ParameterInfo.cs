using System;

namespace LooCast.Command
{
    using LooCast.System;
    
    public sealed class ParameterInfo
    {
        #region Properties
        public string ParameterName { get; }
        public ParameterVariant ParameterVariant { get; }
        public TypeInfo ParameterType { get; }
        #endregion

        #region Constructors
        public ParameterInfo(string parameterName, ParameterVariant parameterVariant, TypeInfo parameterType)
        {
            if (parameterName == null)
            {
                throw new ArgumentNullException(nameof(parameterName));
            }
            if (parameterType == null)
            {
                throw new ArgumentNullException(nameof(parameterType));
            }
            if (!StringUtil.IsAlphaNumeric(parameterName))
            {
                throw new ArgumentException($"Parameter name '{parameterName}' is not alphanumeric!");
            }

            ParameterName = parameterName;
            ParameterVariant = parameterVariant;
            ParameterType = parameterType;
        }
        #endregion
    }
}
