using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    public sealed class MainManagerMonoBehaviour : ExtendedMonoBehaviour
    {
        #region Static Properties
        public static MainManagerMonoBehaviour Instance
        {
            get
            {
                if (instance == null)
                {
                    MainManagerMonoBehaviour mainManagerMonoBehaviour = new GameObject("[MainManager]").AddComponent<MainManagerMonoBehaviour>();
                    mainManagerMonoBehaviour.gameObject.layer = 31;
                    mainManagerMonoBehaviour.gameObject.tag = "INTERNAL";
                    DontDestroyOnLoad(mainManagerMonoBehaviour);
                    instance = mainManagerMonoBehaviour;
                }
                return instance;
            }
        }
        #endregion
        
        #region Static Fields
        private static MainManagerMonoBehaviour instance;
        #endregion

        #region Methods
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreAwake()
        {
            MainManager.Instance.OnPreAwake();
            _ = Instance;
        }
        
        private void Awake()
        {
            MainManager.Instance.OnAwake();
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void PostAwake()
        {
            MainManager.Instance.OnPostAwake();
        }
        #endregion
    }
}
