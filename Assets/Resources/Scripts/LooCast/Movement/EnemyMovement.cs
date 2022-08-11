using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using Data;
    using LooCast.Core;
    using LooCast.Target;
    using LooCast.Variable;
    using LooCast.Util;

    public class EnemyMovement : ExtendedMonoBehaviour, ITargetedMovement
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
        private bool isMovementEnabled;
        private GameObject playerObject;
        private CircleCollider2D playerCollider;
        private Target target;
        #endregion

        #region Methods
        private void Start()
        {
            Speed = new FloatComputedVariable(Data.BaseSpeed.Value);
            Speed.AddPermanentMultiplier(Constants.INERTIAL_COEFFICIENT);
            Speed.AddPermanentMultiplier(UnityEngine.Random.Range(0.9f, 1.1f));
            Rigidbody = GetComponent<Rigidbody2D>();
            Collider = GetComponent<Collider2D>();

            OnMovementEnabled = new UnityEvent();
            OnMovementDisabled = new UnityEvent();

            isMovementEnabled = Data.IsMovementEnabled.Value;
            playerObject = GameObject.FindGameObjectWithTag("Player");
            playerCollider = playerObject.GetComponent<CircleCollider2D>();
            SetTarget(new Target(playerCollider));
        }

        protected override void OnPauseableFixedUpdate()
        {
            Accelerate();
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

        public void Accelerate()
        {
            if (isMovementEnabled)
            {
                Rigidbody.AddForce((target.transform.position - transform.position).normalized * Speed.Value);

                Vector2 lookDir = target.transform.position - transform.position;
                float angle = Mathf.Atan2(lookDir.y, lookDir.x) * Mathf.Rad2Deg - 90.0f;
                transform.rotation = Quaternion.Euler(transform.rotation.eulerAngles.x, transform.rotation.eulerAngles.y, angle);
            }
        }

        public Target GetTarget()
        {
            return target;
        }

        public void SetTarget(Target target)
        {
            this.target = target;
        }
        #endregion
    }
}
