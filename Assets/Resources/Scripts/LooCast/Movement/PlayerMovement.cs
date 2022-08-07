using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using Manager;
    using Data;
    using Data.Runtime;
    using Variable;
    using Attribute.Stat;

    public class PlayerMovement : Movement
    {
        public PlayerMovementData Data;
        public PlayerMovementRuntimeData RuntimeData;

        public Stats Stats;

        [HideInInspector] public UnityEvent OnStartAccelerating;
        [HideInInspector] public UnityEvent OnStopAccelerating;

        private void Start()
        {
            Initialize(Data);

            RuntimeData.CurrentEnergy = new FloatVariable(Data.BaseEnergy.Value);
            RuntimeData.EnergyConsumption = new FloatComputedVariable(Data.BaseEnergyConsumption.Value);
            RuntimeData.EnergyConsumption.AddPermanentMultiplier(Stats.EnergyConsumptionMultiplier);
            RuntimeData.EnergyGeneration = new FloatComputedVariable(Data.BaseEnergyGeneration.Value);
            RuntimeData.EnergyGeneration.AddPermanentMultiplier(Stats.EnergyRegenerationMultiplier);
            RuntimeData.IsUsingEnergy = new BoolVariable(Data.BaseIsUsingEnergy.Value);
            RuntimeData.IsEnergyDepleted = new BoolVariable(Data.BaseIsEnergyDepleted.Value);

            Speed.AddPermanentMultiplier(Stats.MovementSpeedMultiplier);
        }

        protected override void OnPauseableUpdate()
        {
            base.OnPauseableUpdate();
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

        public override void Accelerate()
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
    } 
}
