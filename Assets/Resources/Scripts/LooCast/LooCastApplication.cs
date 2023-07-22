using UnityEngine;

namespace LooCast
{
    using LooCast.System;
    
    public static class LooCastApplication
    {
        #region Static Methods
        public static void Exit()
        {
            MainManager.Instance.OnDestroy();
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreAwake()
        {
            MainManager.Instance.OnPreSetup();
            MainManager.Instance.OnSetup();
            MainManager.Instance.OnPostSetup();
            MainManager.Instance.OnPreAwake();
            MainManager.Instance.OnAwake();
            MainManager.Instance.OnPostAwake();
        }
        #endregion
    }
}
