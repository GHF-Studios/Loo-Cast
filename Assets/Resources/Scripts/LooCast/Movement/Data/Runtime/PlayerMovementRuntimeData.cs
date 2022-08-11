using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement.Data.Runtime
{
    using LooCast.Variable;
    using Attribute.Stat;
    using LooCast.Util;

    [CreateAssetMenu(fileName = "PlayerMovementRuntimeData", menuName = "Data/Movement/Runtime/PlayerMovementRuntimeData", order = 0)]
    public sealed class PlayerMovementRuntimeData : ScriptableObject
    {
        public Stats Stats;

        public void Initialize(PlayerMovementData data)
        {
            CurrentEnergy = new FloatVariable(data.BaseEnergy.Value);
            EnergyConsumption = new FloatComputedVariable(data.BaseEnergyConsumption.Value);
            EnergyConsumption.AddPermanentMultiplier(Stats.EnergyConsumptionMultiplier);
            EnergyGeneration = new FloatComputedVariable(data.BaseEnergyGeneration.Value);
            EnergyGeneration.AddPermanentMultiplier(Stats.EnergyRegenerationMultiplier);
            IsUsingEnergy = new BoolVariable(data.BaseIsUsingEnergy.Value);
            IsEnergyDepleted = new BoolVariable(data.BaseIsEnergyDepleted.Value);
            Speed = new FloatComputedVariable(data.BaseSpeed.Value);
            Speed.AddPermanentMultiplier(Constants.INERTIAL_COEFFICIENT);
            Speed.AddPermanentMultiplier(Stats.MovementSpeedMultiplier);
        }

        public FloatComputedVariable Speed;
        public FloatVariable CurrentEnergy;
        public FloatComputedVariable MaxEnergy;
        public FloatComputedVariable EnergyConsumption;
        public FloatComputedVariable EnergyGeneration;
        public BoolVariable IsUsingEnergy;
        public BoolVariable IsEnergyDepleted;
    }
}
