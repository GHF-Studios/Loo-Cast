using System;
using System.Collections.Generic;

namespace LooCast.System.Lua
{
    public sealed class LuaNamespaceInfo
    {
        #region Properties
        public LuaNamespaceInfo[] ParentNamespaces { get; }
        public string Namespace { get; }
        public Dictionary<string, LuaMethodInfo> Methods { get; }
        #endregion

        #region Constructors
        public LuaNamespaceInfo(LuaNamespaceInfo[] parentNamespaces, string _namespace)
        {
            ParentNamespaces = parentNamespaces;
            Namespace = _namespace;
            Methods = new Dictionary<string, LuaMethodInfo>();
        }
        #endregion
    }
}
