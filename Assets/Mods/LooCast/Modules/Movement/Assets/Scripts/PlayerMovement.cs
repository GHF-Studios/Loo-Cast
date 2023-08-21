using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using Data;
    using Data.Runtime;
    using LooCast.Core;
    using LooCast.Game;
    using LooCast.Variable;

    [DisallowMultipleComponent]
    [RequireComponent(typeof(Rigidbody2D), typeof(Collider2D))]
    public sealed class PlayerMovement : Component, IMovement
    {
        #region Data
        public PlayerMovementData Data;
        public PlayerMovementRuntimeData RuntimeData;
        #endregion

        #region Properties
        public FloatComputedVariable Speed
        {
            get
            {
                return RuntimeData.Speed;
            }
        }
        public Rigidbody2D Rigidbody { get; private set; }
        public Collider2D Collider { get; private set; }
        #endregion

        #region Events
        public UnityEvent OnMovementEnabled;
        public UnityEvent OnMovementDisabled;
        public UnityEvent OnStartAccelerating;
        public UnityEvent OnStopAccelerating;
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

        protected override void PauseableFixedUpdate()
        {
            RuntimeData.IsUsingEnergy.Value = false;

            float[] inputAxis = new float[2];
            inputAxis[0] = Input.GetAxis("Horizontal");
            inputAxis[1] = Input.GetAxis("Vertical");

            #region Acceleration Event Invocation Logic
            if ((inputAxis[0] == 0 && inputAxis[1] == 0) || RuntimeData.IsEnergyDepleted.Value)
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
            #endregion

            if (!RuntimeData.IsEnergyDepleted.Value)
            {
                Vector3 targetDirection = new Vector3(inputAxis[0], inputAxis[1]).normalized;
                AccelerateInDirection(targetDirection);
                LookInDirection(targetDirection); 
            }
        }

        protected override void PauseableUpdate()
        {
            #region Energy Logic

            #region Generation
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
            #endregion
            
            #region Consumption
            if (RuntimeData.IsUsingEnergy.Value && !RuntimeData.IsEnergyDepleted.Value)
            {
                if (RuntimeData.CurrentEnergy.Value - RuntimeData.EnergyConsumption.Value <= 0.0f)
                {
                    RuntimeData.CurrentEnergy.Value = 0.0f;
                    RuntimeData.IsEnergyDepleted.Value = true;
                    GameManager.Instance.gameSoundHandler.SoundCooldown();
                }
                else
                {
                    RuntimeData.CurrentEnergy.Value -= RuntimeData.EnergyConsumption.Value;
                }
            }
            #endregion

            #endregion
        }

        public void AccelerateInDirection(Vector3 targetDirection)
        {
            Rigidbody.AddForce(targetDirection * RuntimeData.Speed.Value);
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
