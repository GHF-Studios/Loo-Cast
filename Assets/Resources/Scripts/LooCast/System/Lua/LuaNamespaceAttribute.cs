using System;

namespace LooCast.System.Lua
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public sealed class LuaNamespaceAttribute : Attribute
    {
        #region Properties
        public string Namespace { get; }
        #endregion

        #region Constructors
        public LuaNamespaceAttribute(string _namespace)
        {
            Namespace = _namespace;
        }
        #endregion
    }
}
