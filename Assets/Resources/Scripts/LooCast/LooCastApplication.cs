using UnityEngine;
using System;

namespace LooCast
{
    using LooCast.System;

    public static class LooCastApplication
    {
        #region Static Properties
        public static event Action<string> OnLogUpdated;
        #endregion

        #region Static Fields
        public static string Log { get; private set; }
        #endregion

        #region Static Methods
        public static void Exit()
        {
            MainManager.Instance.OnDestroy();
            
            Application.logMessageReceived -= UpdateLog_INTERNAL;
        }

        public static void UpdateLog(string logString)
        {
            Log += logString + "\n";
            if (OnLogUpdated != null)
            {
                OnLogUpdated.Invoke(Log);
            }
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreAwake()
        {
            Application.logMessageReceived += UpdateLog_INTERNAL;

            MainManager.Instance.OnPreSetup();
            MainManager.Instance.OnSetup();
            MainManager.Instance.OnPostSetup();
            MainManager.Instance.OnPreAwake();
            MainManager.Instance.OnAwake();
            MainManager.Instance.OnPostAwake();
        }

        private static void UpdateLog_INTERNAL(string logString, string stackTrace, LogType type)
        {
            Log += logString + "\n";
            if (OnLogUpdated != null)
            {
                OnLogUpdated.Invoke(Log);
            }
        }
        #endregion
    }
}
