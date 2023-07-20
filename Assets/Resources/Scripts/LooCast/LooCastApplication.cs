using UnityEngine;

namespace LooCast
{
    using LooCast.System;
    
    public static class LooCastApplication
    {
        #region Methods
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreAwake()
        {
            MainManager.Instance.OnPreAwake();
            MainManager.Instance.OnAwake();
            MainManager.Instance.OnPostAwake();
        }
        #endregion
    }
}
