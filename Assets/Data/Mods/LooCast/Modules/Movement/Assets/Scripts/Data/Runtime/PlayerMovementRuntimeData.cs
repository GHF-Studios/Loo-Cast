using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement.Data.Runtime
{
    using LooCast.Variable;
    using Attribute.Stat;
    using LooCast.Util;
    using LooCast.Event;

    [CreateAssetMenu(fileName = "PlayerMovementRuntimeData", menuName = "Data/Movement/Runtime/PlayerMovementRuntimeData", order = 0)]
    public sealed class PlayerMovementRuntimeData : ScriptableObject
    {
        public Stats Stats;

        public void Initialize(PlayerMovementData data)
        {
            Speed = new FloatComputedVariable(data.BaseSpeed.Value);
            Speed.AddPermanentMultiplier(Constants.InertialCoefficient);
            Speed.AddPermanentMultiplier(Stats.MovementSpeedMultiplier);
            MaxEnergy = new FloatComputedVariable(data.BaseMaxEnergy.Value);
            MaxEnergy.AddPermanentMultiplier(Stats.EnergyMultiplier);
            CurrentEnergy = new FloatVariable(MaxEnergy.Value);
            EnergyConsumption = new FloatComputedVariable(data.BaseEnergyConsumption.Value);
            EnergyConsumption.AddPermanentMultiplier(Stats.EnergyConsumptionMultiplier);
            EnergyGeneration = new FloatComputedVariable(data.BaseEnergyGeneration.Value);
            EnergyGeneration.AddPermanentMultiplier(Stats.EnergyRegenerationMultiplier);
            IsUsingEnergy = new BoolVariable(data.BaseIsUsingEnergy.Value);
            IsEnergyDepleted = new BoolVariable(data.BaseIsEnergyDepleted.Value);
        }

        public FloatComputedVariable Speed;
        public FloatComputedVariable MaxEnergy;
        public FloatVariable CurrentEnergy;
        public FloatComputedVariable EnergyConsumption;
        public FloatComputedVariable EnergyGeneration;
        public BoolVariable IsUsingEnergy;
        public BoolVariable IsEnergyDepleted;
    }
}
