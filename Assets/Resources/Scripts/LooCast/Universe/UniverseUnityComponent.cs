using System;
using UnityEngine;

namespace LooCast.Universe
{
    using LooCast.Core;
    
    public sealed class UniverseUnityComponent : UnityComponent
    {
        #region Properties
        public Universe Universe { get; private set; }
        #endregion

        #region Methods
        public void InitializeUniverse(Universe universe)
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
