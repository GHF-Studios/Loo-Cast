using System;

namespace LooCast.System
{
    public sealed class ManagerObject : GameObject
    {
        #region Static Methods
#nullable enable
        public static ManagerObject CreateManagerObject()
        {
            return CreateGameObject<ManagerObject>();
        }
#nullable disable
        #endregion

        #region Overrides
        protected override void PreConstruct()
        {
            base.PreConstruct();

            UnityEngine.GameObject.DontDestroyOnLoad(UnityEngineGameObject);
        }
        #endregion
    }
}
