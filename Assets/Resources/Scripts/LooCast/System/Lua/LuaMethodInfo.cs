using System.Reflection;

namespace LooCast.System.Lua
{
    public sealed class LuaMethodInfo
    {
        #region Propertiess
        public LuaNamespaceInfo Namespace { get; }
        public string MethodName { get; }
        public MethodInfo MethodInfo { get; }
        #endregion

        #region Constructors
        public LuaMethodInfo(LuaNamespaceInfo _namespace, string methodName, MethodInfo methodInfo)
        {
            Namespace = _namespace;
            MethodName = methodName;
            MethodInfo = methodInfo;
        }
        #endregion
    }
}
