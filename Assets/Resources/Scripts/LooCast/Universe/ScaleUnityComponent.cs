using System;
using UnityEngine;

namespace LooCast.Universe
{
    using LooCast.Core;
    
    public sealed class ScaleUnityComponent : UnityComponent
    {
        #region Properties
        public Scale Scale { get; private set; }
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
