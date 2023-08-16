using System;

namespace LooCast.Command
{
    [AttributeUsage(AttributeTargets.Class, Inherited = false, AllowMultiple = false)]
    public sealed class CommandNamespaceAttribute : Attribute
    {
        public string Name { get; }

        public CommandNamespaceAttribute(string name)
        {
            Name = name;
        }
    }
}
