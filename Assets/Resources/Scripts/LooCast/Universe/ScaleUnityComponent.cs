using System;
using UnityEngine;

namespace LooCast.Universe
{
    using global::LooCast.System.ECS;
    
    public sealed class ScaleUnityComponent : UnityComponent
    {
        #region Properties
        public Scale Scale { get; private set; }
        #endregion

        #region Unity Callbacks
        private void FixedUpdate()
        {
            if (Scale is not null)
            {
                transform.localScale = Vector3.one * Scale.ScaleFactor;
            }
        }
        #endregion

        #region Methods
        public void InitializeScale(Scale scale)
        {
            if (Scale is not null)
            {
                throw new InvalidOperationException($"Scale reference has already been initialized!");
            }

            Scale = scale;
        }
        #endregion
    }
}
