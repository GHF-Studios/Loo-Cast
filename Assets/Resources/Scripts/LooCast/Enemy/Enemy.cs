using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Enemy
{
    using Data;
    using Data.Runtime;
    using Core;
    using Particle;
    using Movement;
    using Health;

    [RequireComponent(typeof(EnemyMovement), typeof(EnemyHealth)), DisallowMultipleComponent]
    public abstract class Enemy : Component
    {
        #region Data
        public EnemyData Data;
        public EnemyRuntimeSet RuntimeSet;
        #endregion

        #region Properties
        public EnemyMovement Movement { get; private set; }
        public EnemyHealth Health { get; private set; }
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

            Movement = GetComponent<EnemyMovement>();
            Health = GetComponent<EnemyHealth>();
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
