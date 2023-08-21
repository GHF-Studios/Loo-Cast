using System;
using UnityEngine;

namespace LooCast.Universe
{
    using global::LooCast.System.ECS;
    
    public sealed class UniverseUnityComponent : UnityComponent
    {
        #region Properties
        public Universe Universe { get; private set; }
        #endregion

        #region Methods
        public void Setup(Universe universe)
        {
            if (Universe is not null)
            {
                throw new InvalidOperationException($"Universe reference has already been initialized!");
            }

            Universe = universe;
        }
        #endregion
    }
}
