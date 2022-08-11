using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Enemy
{
    using Core;
    using Data;
    using Data.Runtime;
    using Particle;
    using Manager;
    using Movement;
    using Health;

    [RequireComponent(typeof(EnemyMovement), typeof(EnemyHealth)), DisallowMultipleComponent]
    public abstract class Enemy : ExtendedMonoBehaviour
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

            OnKilled = new UnityEvent();
        }

        public void Kill()
        {
            OnKilled.Invoke();
            RuntimeSet.Remove(this);
            ParticleSystem.Kill();
            Destroy(gameObject);
        }

        private void OnCollisionStay2D(Collision2D collision)
        {
            if (collision.gameObject.CompareTag("Player"))
            {
                PlayerHealth playerHealth = GameSceneManager.Instance.Player.Health;
                float difficulty;
                if (!PlayerPrefs.HasKey("Difficulty"))
                {
                    PlayerPrefs.SetFloat("Difficulty", 1.0f);
                }
                difficulty = PlayerPrefs.GetFloat("Difficulty");
                playerHealth.Damage(new DamageInfo(collision.gameObject, collision.gameObject, Data.ContactDamage.Value * difficulty, 0, 0, 0, 0));
            }
        }
        #endregion
    } 
}
