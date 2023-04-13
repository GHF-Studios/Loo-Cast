using System;

namespace LooCast.System
{
    using global::LooCast.System.MetaData;
    
    public sealed class ManagerObject : GameObject
    {
        #region Static Methods
#nullable enable
        public static ManagerObject CreateManagerObject()
        {
            return CreateGameObject<ManagerObject, ManagerObjectMetaData>();
        }
#nullable disable

        #region Overrides
        protected override void PreConstruct()
        {
            base.PreConstruct();

            UnityEngine.GameObject.DontDestroyOnLoad(UnityEngineGameObject);
        }
        #endregion
    }
}
