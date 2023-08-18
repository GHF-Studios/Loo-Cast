using UnityEngine;
using System;

namespace LooCast
{
    using LooCast.System;
    using LooCast.System.Lua;

    [LuaNamespace("LooCast")]
    public static class LooCastApplication
    {
        #region Static Properties
        public static event Action OnLogUpdated;
        #endregion

        #region Static Fields
        public static string LogHistory { get; private set; }
        #endregion

        #region Static Methods
        public static void Exit()
        {
            MainManager.Instance.OnDestroy();
            
            Application.logMessageReceived -= Log_INTERNAL;
        }

        [LuaMethod("Log")]
        public static void Log(string logString)
        {
            LogHistory += logString + "\n";
            if (OnLogUpdated != null)
            {
                OnLogUpdated.Invoke();
            }
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreAwake()
        {
            Application.logMessageReceived += Log_INTERNAL;

            MainManager.Instance.OnPreSetup();
            MainManager.Instance.OnSetup();
            MainManager.Instance.OnPostSetup();
            MainManager.Instance.OnPreAwake();
            MainManager.Instance.OnAwake();
            MainManager.Instance.OnPostAwake();
        }

        private static void Log_INTERNAL(string logString, string stackTrace, LogType type)
        {
            LogHistory += logString + "\n";
            if (OnLogUpdated != null)
            {
                OnLogUpdated.Invoke();
            }
        }
        #endregion
    }
}
