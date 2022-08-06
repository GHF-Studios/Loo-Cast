using UnityEngine;

namespace LooCast.Movement.Data.Runtime
{
    using LooCast.Variable;

    [CreateAssetMenu(fileName = "PlayerMovementRuntimeData", menuName = "Data/Movement/PlayerMovementRuntimeData", order = 0)]
    public sealed class PlayerMovementRuntimeData : ScriptableObject
    {
        public FloatVariable CurrentEnergy;
        public FloatComputedVariable MaxEnergy;
        public FloatComputedVariable EnergyConsumption;
        public FloatComputedVariable EnergyGeneration;
        public BoolVariable IsUsingEnergy;
        public BoolVariable IsEnergyDepleted;
    }
}
