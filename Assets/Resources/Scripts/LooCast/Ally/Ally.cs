using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Ally
{
    using Data;
    using Data.Runtime;
    using Core;
    using Particle;
    using Movement;
    using Health;

    [RequireComponent(typeof(AllyMovement), typeof(AllyHealth)), DisallowMultipleComponent]
    public abstract class Ally : GameObject
    {
        #region Data
        public AllyData Data;
        public AllyRuntimeSet RuntimeSet;
        #endregion

        #region Properties
        public AllyMovement Movement { get; private set; }
        public AllyHealth Health { get; private set; }
        public ParticleSystem ParticleSystem { get; private set; }
        #endregion

        #region Events
        public static UnityEvent<Type> OnKillCounted
        {
            get
            {
                return onKillCounted;
            }

            private set
            {
                onKillCounted = value;
            }
        }
        public UnityEvent OnKilled
        {
            get
            {
                return onKilled;
            }

            set
            {
                onKilled = value;
            }
        }

        [SerializeField] private static UnityEvent<Type> onKillCounted = new UnityEvent<Type>();
        [SerializeField] private UnityEvent onKilled;
        #endregion

        #region Fields
        #endregion

        #region Methods
        private void Start()
        {
            RuntimeSet.Add(this);

            Movement = GetComponent<AllyMovement>();
            Health = GetComponent<AllyHealth>();
            ParticleSystem = GetComponentInChildren<ParticleSystem>();
        }

        public void Kill()
        {
            OnKilled.Invoke();
            OnKillCounted.Invoke(GetType());
            RuntimeSet.Remove(this);
            ParticleSystem.Kill();
            Destroy(gameObject);
        }
        #endregion
    } 
}
