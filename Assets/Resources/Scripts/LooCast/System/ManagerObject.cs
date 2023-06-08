using System;
using UnityEngine;

namespace LooCast.System
{
    public sealed class ManagerObject : MonoBehaviour
    {
        #region Properties
        #endregion

        #region Static Methods
#nullable enable
        public static ManagerObject CreateManagerObject()
        {

        }
#nullable disable
        #endregion

        #region Overrides
        public override bool Validate()
        {
            return true;
        }
        
        protected override void PreConstruct()
        {
            base.PreConstruct();

            UnityEngine.GameObject.DontDestroyOnLoad(UnityEngineGameObject);
        }
        #endregion
    }
}
