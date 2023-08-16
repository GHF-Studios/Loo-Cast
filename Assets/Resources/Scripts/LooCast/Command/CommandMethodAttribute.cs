using System;

namespace LooCast.Command
{
    [AttributeUsage(AttributeTargets.Method, Inherited = false, AllowMultiple = false)]
    public sealed class CommandMethodAttribute : Attribute
    {
        public string Name { get; }

        public CommandMethodAttribute(string name)
        {
            Name = name;
        }
    }
}
