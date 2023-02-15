using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using LooCast.System;
    using Data;
    using LooCast.Core;
    using LooCast.Target;
    using LooCast.Variable;
    using LooCast.Util;

    public class EnemyMovement : ExtendedMonoBehaviour, IMovement
    {
        #region Data
        public EnemyMovementData Data;
        #endregion

        #region Properties
        public FloatComputedVariable Speed { get; private set; }
        public Rigidbody2D Rigidbody { get; private set; }
        public Collider2D Collider { get; private set; }
        #endregion

        #region Events
        public UnityEvent OnMovementEnabled
        {
            get
            {
                return onMovementEnabled;
            }

            set
            {
                onMovementEnabled = value;
            }
        }
        [SerializeField] private UnityEvent onMovementEnabled;
        public UnityEvent OnMovementDisabled
        {
            get
            {
                return onMovementDisabled;
            }

            set
            {
                onMovementDisabled = value;
            }
        }
        [SerializeField] private UnityEvent onMovementDisabled;
        #endregion

        #region Fields
        private Vector3 PAUSE_currentVelocity;
        #endregion

        #region Methods
        private void Awake()
        {
            Speed = new FloatComputedVariable(Data.BaseSpeed.Value);

            Rigidbody = GetComponent<Rigidbody2D>();
            Collider = GetComponent<Collider2D>();
        }

        private void Start()
        {
            Speed.AddPermanentMultiplier(Constants.InertialCoefficient);
            Speed.AddPermanentMultiplier(UnityEngine.Random.Range(0.9f, 1.1f));
        }

        protected override void OnPause()
        {
            PAUSE_currentVelocity = Rigidbody.velocity;
            Rigidbody.velocity = Vector3.zero;
        }

        protected override void OnResume()
        {
            Rigidbody.velocity = PAUSE_currentVelocity;
            PAUSE_currentVelocity = Vector3.zero;
        }

        public void AccelerateInDirection(Vector3 targetDirection)
        {
            Rigidbody.AddForce(targetDirection * Speed.Value);
        }

        public void LookInDirection(Vector3 targetDirection)
        {
            float angle = Mathf.Atan2(targetDirection.y, targetDirection.x) * Mathf.Rad2Deg - 90.0f;
            transform.rotation = Quaternion.Euler(transform.rotation.eulerAngles.x, transform.rotation.eulerAngles.y, angle);
        }

        public void AccelerateToPosition(Vector3 targetPosition)
        {
            Vector3 targetDirection = (targetPosition - transform.position).normalized;
            AccelerateInDirection(targetDirection);
        }

        public void LookAtPosition(Vector3 targetPosition)
        {
            Vector3 targetDirection = (targetPosition - transform.position).normalized;
            LookInDirection(targetDirection);
        }
        #endregion
    }
}
