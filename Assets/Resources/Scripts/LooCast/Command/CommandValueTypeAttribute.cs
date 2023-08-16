using System;

namespace LooCast.Command
{
    [AttributeUsage(AttributeTargets.Struct, Inherited = false, AllowMultiple = false)]
    public sealed class CommandValueTypeAttribute : CommandTypeAttribute
    {
        public string Name { get; }

        public CommandValueTypeAttribute(string name)
        {
            Name = name;
        }
    }
}
