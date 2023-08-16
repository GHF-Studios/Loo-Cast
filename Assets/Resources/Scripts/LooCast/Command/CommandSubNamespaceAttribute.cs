using System;

namespace LooCast.Command
{
    [AttributeUsage(AttributeTargets.Class, Inherited = false, AllowMultiple = false)]
    public sealed class CommandSubNamespaceAttribute : Attribute
    {
        public string Name { get; }

        public CommandSubNamespaceAttribute(string name)
        {
            Name = name;
        }
    }
}
