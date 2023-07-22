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
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreAwake()
        {
            Entity entity = new Entity();
            Debug.LogError(entity);

            Manager manager = new Manager();
            Debug.LogError(manager);
        }
        #endregion
    }
}
