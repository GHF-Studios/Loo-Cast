using System;

namespace LooCast.System.Lua
{
    [AttributeUsage(AttributeTargets.Method, AllowMultiple = false)]
    public sealed class LuaMethodAttribute : Attribute
    {
        #region Properties
        public string LuaMethodName { get; }
        #endregion

        #region Constructors
        public LuaMethodAttribute(string luaMethodName)
        {
            LuaMethodName = luaMethodName;
        }
        #endregion
    }
}
