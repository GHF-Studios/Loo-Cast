using UnityEngine;

namespace LooCast
{
    using LooCast.System;
    using LooCast.System.ECS;
    using LooCast.System.Paths;

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
            Entity entity = new Entity();
            Debug.LogError(entity);

            Manager manager = new Manager();
            Debug.LogError(manager);

            MainManager mainManager = new MainManager();
            Debug.LogError(mainManager);

            // MainManager.Instance.OnPreSetup();
            // MainManager.Instance.OnSetup();
            // MainManager.Instance.OnPostSetup();
            // MainManager.Instance.OnPreAwake();
            // MainManager.Instance.OnAwake();
            // MainManager.Instance.OnPostAwake();
        }
        #endregion
    }
}
