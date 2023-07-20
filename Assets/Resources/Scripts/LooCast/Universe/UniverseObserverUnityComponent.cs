using System;
using System.Collections;
using UnityEngine;

namespace LooCast.Universe
{
    using global::LooCast.System.Numerics;
    using global::LooCast.System.ECS;
    
    public sealed class UniverseObserverUnityComponent : UnityComponent
    {
        #region Properties
        public UniverseObserver UniverseObserver { get; private set; }
        #endregion

        #region Unity Callbacks
        private void Update()
        {
            UniverseObserver.Update();

            if (Input.GetKeyDown(KeyCode.KeypadMinus))
            {
                if (!UniverseObserver.IsTransitioningScale)
                {
                    TransitionScale(UniverseObserver.ScaleTransitionType.ZoomIn);
                }
            }
            else if (Input.GetKeyDown(KeyCode.KeypadPlus))
            {
                if (!UniverseObserver.IsTransitioningScale)
                {
                    TransitionScale(UniverseObserver.ScaleTransitionType.ZoomOut);
                }
            }
        }

        private void OnDrawGizmos()
        {
            if (UniverseObserver is not null)
            {
                Gizmos.color = Color.blue;
                Gizmos.DrawWireSphere(transform.position, UniverseObserver.ObservingDistance);
            }
        }
        #endregion

        #region Coroutines
        private IEnumerator TransitionScaleCoroutine(UniverseObserver.ScaleTransitionType scaleTransitionType)
        {
            float elapsedTime = 0f;

            UniverseObserver.InitializeScaleTransition(scaleTransitionType);

            while (elapsedTime < UniverseObserver.ScaleTransitionDuration)
            {
                elapsedTime += Time.deltaTime;
                float t = Mathf.Clamp01(elapsedTime / UniverseObserver.ScaleTransitionDuration);

                float oldScaleFactor;
                float oldAlphaFactor = Mathf.Lerp(1.0f, 0.0f, t);
                float newScaleFactor;
                float newAlphaFactor = Mathf.Lerp(0.0f, 1.0f, t);

                if (scaleTransitionType == UniverseObserver.ScaleTransitionType.ZoomIn)
                {
                    oldScaleFactor = Mathf.Lerp(1.0f, 10.0f, t);
                    newScaleFactor = Mathf.Lerp(0.1f, 1.0f, t);
                }
                else
                {
                    oldScaleFactor = Mathf.Lerp(1.0f, 0.1f, t);
                    newScaleFactor = Mathf.Lerp(10.0f, 1.0f, t);
                }
                

                UniverseObserver.UpdateScaleTransition(oldScaleFactor, oldAlphaFactor, newScaleFactor, newAlphaFactor);

                yield return null;
            }
            
            UniverseObserver.FinalizeScaleTransition();
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

        private void TransitionScale(UniverseObserver.ScaleTransitionType scaleTransitionType)
        {
            StartCoroutine(TransitionScaleCoroutine(scaleTransitionType));
        }
        #endregion
    }
}
