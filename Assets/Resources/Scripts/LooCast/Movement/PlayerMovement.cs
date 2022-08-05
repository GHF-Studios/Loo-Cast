using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using Manager;
    using Data;
    using Data.Runtime;
    using Attribute.Stat;

    public class PlayerMovement : Movement
    {
        public PlayerMovementData Data;
        public PlayerMovementRuntimeData RuntimeData;

        public Stats Stats;

        public UnityEvent OnStartAccelerating;
        public UnityEvent OnStopAccelerating;

        private void Start()
        {
            Initialize(Data);

            RuntimeData.CurrentEnergy = Data.BaseEnergy.Value;
            RuntimeData.EnergyConsumption = Data.BaseEnergyConsumption.Value * Stats.EnergyConsumptionMultiplier;
            RuntimeData.EnergyGeneration = Data.BaseEnergyGeneration.Value * Stats.EnergyRegenerationMultiplier;
            RuntimeData.IsUsingEnergy = Data.IsUsingEnergy.Value;
            RuntimeData.IsEnergyDepleted = Data.IsEnergyDepleted.Value;

            Speed.AddPermanentMultiplier(Stats.MovementSpeedMultiplier);
        }

        protected override void OnPauseableUpdate()
        {
            base.OnPauseableUpdate();
            if (!RuntimeData.IsUsingEnergy)
            {
                if (RuntimeData.CurrentEnergy + RuntimeData.EnergyGeneration >= RuntimeData.MaxEnergy)
                {
                    RuntimeData.CurrentEnergy = RuntimeData.MaxEnergy;
                    RuntimeData.IsEnergyDepleted = false;
                }
                else
                {
                    RuntimeData.CurrentEnergy += RuntimeData.EnergyGeneration;
                }
            }
            if (RuntimeData.IsUsingEnergy && !RuntimeData.IsEnergyDepleted)
            {
                if (RuntimeData.CurrentEnergy - RuntimeData.EnergyConsumption <= 0.0f)
                {
                    RuntimeData.CurrentEnergy = 0.0f;
                    RuntimeData.IsEnergyDepleted = true;
                    GameSceneManager.Instance.SoundHandler.SoundCooldown();
                }
                else
                {
                    RuntimeData.CurrentEnergy -= RuntimeData.EnergyConsumption;
                }
            }
        }

        public override void Accelerate()
        {
            RuntimeData.IsUsingEnergy = false;
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

            if ((axis[0] == 0 && axis[1] == 0) || RuntimeData.IsEnergyDepleted)
            {
                OnStopAccelerating.Invoke();
            }
            else
            {
                OnStartAccelerating.Invoke();
                if (!RuntimeData.IsEnergyDepleted)
                {
                    RuntimeData.IsUsingEnergy = true;
                }
            }

            if (!RuntimeData.IsEnergyDepleted)
            {
                Rigidbody.AddForce(new Vector2(axis[0], axis[1]).normalized * Speed.Value); 
            }
        }
    } 
}
