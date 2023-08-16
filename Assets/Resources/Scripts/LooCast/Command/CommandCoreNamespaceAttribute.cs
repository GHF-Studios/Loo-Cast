using System;

namespace LooCast.Command
{
    [AttributeUsage(AttributeTargets.Class, Inherited = false, AllowMultiple = false)]
    public sealed class CommandCoreNamespaceAttribute : Attribute
    {
        public string Name { get; }

        public CommandCoreNamespaceAttribute(string name)
        {
            Name = name;
        }
    }
}
