using UnityEngine;

namespace LooCast.Movement.Data.Runtime
{
    [CreateAssetMenu(fileName = "PlayerMovementRuntimeData", menuName = "Data/Movement/PlayerMovementRuntimeData", order = 0)]
    public sealed class PlayerMovementRuntimeData : ScriptableObject
    {
        public float CurrentEnergy;
        public float MaxEnergy;
        public float EnergyConsumption;
        public float EnergyGeneration;
        public bool IsUsingEnergy;
        public bool IsEnergyDepleted;
    }
}
