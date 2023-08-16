using System;

namespace LooCast.Command
{
    using LooCast.System;
    
    public sealed class MethodDocumentation
    {
        #region Properties
        public string Description { get; }
        public string Usage { get; }
        public string[] Examples { get; }
        public string[] Parameters { get; }
        public string ReturnValue { get; }
        public string[] Errors { get; }
        #endregion

        #region Constructors
        public MethodDocumentation(string description, string usage, string[] examples, string[] parameters, string returnValue, string[] errors)
        {
            if (StringUtil.IsEmpty(description))
            {
                throw new ArgumentException("Description cannot be empty!", nameof(description));
            }
            if (StringUtil.IsEmpty(usage))
            {
                throw new ArgumentException("Usage cannot be empty!", nameof(usage));
            }
            if (examples == null)
            {
                throw new ArgumentNullException(nameof(examples));
            }
            if (parameters == null)
            {
                throw new ArgumentNullException(nameof(parameters));
            }
            if (StringUtil.IsEmpty(returnValue))
            {
                throw new ArgumentException("Return value cannot be empty!", nameof(returnValue));
            }
            if (errors == null)
            {
                throw new ArgumentNullException(nameof(errors));
            }
            foreach (string example in examples)
            {
                if (StringUtil.IsEmpty(example))
                {
                    throw new ArgumentException("Example cannot be empty!", nameof(examples));
                }
            }
            foreach (string parameter in parameters)
            {
                if (StringUtil.IsEmpty(parameter))
                {
                    throw new ArgumentException("Parameter cannot be empty!", nameof(parameters));
                }
            }
            foreach (string error in errors)
            {
                if (StringUtil.IsEmpty(error))
                {
                    throw new ArgumentException("Error cannot be empty!", nameof(errors));
                }
            }

            Description = description;
            Usage = usage;
            Examples = examples;
            Parameters = parameters;
            ReturnValue = returnValue;
            Errors = errors;
        }
        #endregion
    }
}
