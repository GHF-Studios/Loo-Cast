using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    public class ExtendedMonoBehaviour : MonoBehaviour
    {
        public static List<ExtendedMonoBehaviour> Instances = new List<ExtendedMonoBehaviour>();
        public static int nextInstanceID = 0;
        public int InstanceID { get; private set; }
        public bool IsPaused { get; private set; }
        [HideInInspector] public bool IsVisible;

        private void Awake()
        {
            Instances.Add(this);
            InstanceID = nextInstanceID;
            nextInstanceID++;
            IsPaused = false;
            IsVisible = false;
        }

        private void OnDestroy()
        {
            Instances.Remove(this);
        }

        private void Update()
        {
            if (!IsPaused && enabled && gameObject.activeInHierarchy)
            {
                PauseableUpdate();
            }
        }

        private void FixedUpdate()
        {
            if (!IsPaused && enabled && gameObject.activeInHierarchy)
            {
                PauseableFixedUpdate();
            }
        }

        private void OnBecameInvisible()
        {
            IsVisible = false;
        }

        private void OnBecameVisible()
        {
            IsVisible = true;
        }

        protected virtual void PauseableUpdate()
        {

        }

        protected virtual void PauseableFixedUpdate()
        {

        }

        protected virtual void OnPause()
        {

        }

        protected virtual void OnResume()
        {

        }

        public void Pause()
        {
            OnPause();
            IsPaused = true;
        }
        
        public void Resume()
        {
            OnResume();
            IsPaused = false;
        }
    } 
}
