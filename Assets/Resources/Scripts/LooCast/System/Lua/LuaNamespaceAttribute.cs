using System;

namespace LooCast.System.Lua
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public sealed class LuaNamespaceAttribute : Attribute
    {
        #region Properties
        public string LuaNamespaceName { get; }
        #endregion

        #region Constructors
        public LuaNamespaceAttribute(string luaNamespaceName)
        {
            LuaNamespaceName = luaNamespaceName;
        }
        #endregion
    }
}
