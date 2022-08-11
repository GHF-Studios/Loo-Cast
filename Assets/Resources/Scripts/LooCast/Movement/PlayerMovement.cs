using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using Data;
    using Data.Runtime;
    using LooCast.Core;
    using LooCast.Manager;
    using LooCast.Variable;

    [DisallowMultipleComponent]
    [RequireComponent(typeof(Rigidbody2D), typeof(Collider2D))]
    public sealed class PlayerMovement : ExtendedMonoBehaviour, IMovement
    {
        #region Data
        public PlayerMovementData Data;
        public PlayerMovementRuntimeData RuntimeData;
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
        [SerializeField]private UnityEvent onMovementEnabled;
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
        [SerializeField]private UnityEvent onMovementDisabled;
        public UnityEvent OnStartAccelerating
        {
            get
            {
                return onStartAccelerating;
            }

            set
            {
                onStartAccelerating = value;
            }
        }
        [SerializeField]private UnityEvent onStartAccelerating;
        public UnityEvent OnStopAccelerating
        {
            get
            {
                return onStopAccelerating;
            }

            set
            {
                onStopAccelerating = value;
            }
        }
        [SerializeField]private UnityEvent onStopAccelerating;
        #endregion

        #region Fields
        private Vector3 PAUSE_currentVelocity;
        #endregion

        #region Methods
        private void Start()
        {
            RuntimeData.Initialize(Data);

            Rigidbody = GetComponent<Rigidbody2D>();
            Collider = GetComponent<Collider2D>();

            OnMovementEnabled = new UnityEvent();
            OnMovementDisabled = new UnityEvent();
            OnStartAccelerating = new UnityEvent();
            OnStopAccelerating = new UnityEvent();
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

        protected override void OnPauseableFixedUpdate()
        {
            Accelerate();
        }

        protected override void OnPauseableUpdate()
        {
            if (!RuntimeData.IsUsingEnergy.Value)
            {
                if (RuntimeData.CurrentEnergy.Value + RuntimeData.EnergyGeneration.Value >= RuntimeData.MaxEnergy.Value)
                {
                    RuntimeData.CurrentEnergy.Value = RuntimeData.MaxEnergy.Value;
                    RuntimeData.IsEnergyDepleted.Value = false;
                }
                else
                {
                    RuntimeData.CurrentEnergy.Value += RuntimeData.EnergyGeneration.Value;
                }
            }
            if (RuntimeData.IsUsingEnergy.Value && !RuntimeData.IsEnergyDepleted.Value)
            {
                if (RuntimeData.CurrentEnergy.Value - RuntimeData.EnergyConsumption.Value <= 0.0f)
                {
                    RuntimeData.CurrentEnergy.Value = 0.0f;
                    RuntimeData.IsEnergyDepleted.Value = true;
                    GameSceneManager.Instance.SoundHandler.SoundCooldown();
                }
                else
                {
                    RuntimeData.CurrentEnergy.Value -= RuntimeData.EnergyConsumption.Value;
                }
            }
        }

        public void Accelerate()
        {
            RuntimeData.IsUsingEnergy.Value = false;
            float[] axis = new float[2];
            if (Input.touchCount > 0)
            {
                Vector2 touchPosition = Camera.main.ScreenToWorldPoint(Input.GetTouch(0).position, Camera.MonoOrStereoscopicEye.Mono);
                Vector2 direction = touchPosition - (Vector2)transform.position;
                axis[0] = direction.x;
                axis[1] = direction.y;
            }
            else if (Input.GetMouseButton(0))
            {
                Vector2 mousePosition = Camera.main.ScreenToWorldPoint(Input.mousePosition, Camera.MonoOrStereoscopicEye.Mono);
                Vector2 direction = mousePosition - (Vector2)transform.position;
                axis[0] = direction.x;
                axis[1] = direction.y;
            }
            else
            {
                axis[0] = Input.GetAxis("Horizontal");
                axis[1] = Input.GetAxis("Vertical");
            }

            if ((axis[0] == 0 && axis[1] == 0) || RuntimeData.IsEnergyDepleted.Value)
            {
                OnStopAccelerating.Invoke();
            }
            else
            {
                OnStartAccelerating.Invoke();
                if (!RuntimeData.IsEnergyDepleted.Value)
                {
                    RuntimeData.IsUsingEnergy.Value = true;
                }
            }

            if (!RuntimeData.IsEnergyDepleted.Value)
            {
                Rigidbody.AddForce(new Vector2(axis[0], axis[1]).normalized * Speed.Value); 
            }
        }
        #endregion
    } 
}
