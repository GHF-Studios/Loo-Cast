using UnityEngine;
using System.Collections.Generic;

namespace LooCast.Core
{
    public abstract class ExtendedMonoBehaviour : MonoBehaviour
    {
        public static List<ExtendedMonoBehaviour> Instances = new List<ExtendedMonoBehaviour>();
        public static int nextID = 0;
        [HideInInspector] public int ID;
        public bool IsPaused { get; private set; }
        [HideInInspector] public bool IsVisible;

        private void Awake()
        {
            Instances.Add(this);
            ID = nextID;
            nextID++;
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
