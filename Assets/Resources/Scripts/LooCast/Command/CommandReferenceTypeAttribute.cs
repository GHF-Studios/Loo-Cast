using System;

namespace LooCast.Command
{
    [AttributeUsage(AttributeTargets.Class, Inherited = false, AllowMultiple = false)]
    public sealed class CommandReferenceTypeAttribute : CommandTypeAttribute
    {
        public string Name { get; }

        public CommandReferenceTypeAttribute(string name)
        {
            Name = name;
        }
    }
}
