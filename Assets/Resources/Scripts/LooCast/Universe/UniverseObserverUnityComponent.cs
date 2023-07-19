using System;
using UnityEngine;

namespace LooCast.Universe
{
    using LooCast.System.Numerics;
    using LooCast.Core;
    
    public sealed class UniverseObserverUnityComponent : UnityComponent
    {
        #region Properties
        public UniverseObserver UniverseObserver { get; private set; }
        #endregion

        #region Unity Callbacks
        private void Update()
        {
            UniverseObserver.Update();
        }

        private void OnDrawGizmos()
        {
            if (UniverseObserver is not null)
            {
                Gizmos.color = Color.red;
                Gizmos.DrawWireSphere(transform.position, UniverseObserver.ObservingDistance);
            }
        }
        #endregion

        #region Methods
        public void InitializeUniverseObserver(UniverseObserver universeObserver)
        {
            if (UniverseObserver is not null)
            {
                throw new InvalidOperationException($"UniverseObserver reference has already been initialized!");
            }

            UniverseObserver = universeObserver;
        }
        #endregion
    }
}
