using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    public sealed class MainManagerMonoBehaviour : ExtendedMonoBehaviour
    {
        #region Static Properties
        public static MainManagerMonoBehaviour Instance { get; private set; }
        #endregion

        #region Methods
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreAwake()
        {
            Instance = new GameObject("[MainManager]").AddComponent<MainManagerMonoBehaviour>();
            Instance.gameObject.layer = 31;
            Instance.gameObject.tag = "INTERNAL";
            DontDestroyOnLoad(Instance);
            MainManager.Instance.OnPreAwake();
            MainManager.Instance.OnAwake();
            MainManager.Instance.OnPostAwake();
        }
        #endregion
    }
}
