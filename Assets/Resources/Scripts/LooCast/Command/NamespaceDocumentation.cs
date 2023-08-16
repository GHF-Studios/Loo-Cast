using System;

namespace LooCast.Command
{
    using LooCast.System;

    public sealed class NamespaceDocumentation
    {
        #region Properties
        public string Description { get; }
        public string Usage { get; }
        public string[] Examples { get; }
        #endregion

        #region Constructors
        public NamespaceDocumentation(string description, string usage, string[] examples)
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
            foreach (string example in examples)
            {
                if (StringUtil.IsEmpty(example))
                {
                    throw new ArgumentException("Example cannot be empty!", nameof(examples));
                }
            }

            Description = description;
            Usage = usage;
            Examples = examples;
        }
        #endregion
    }
}
