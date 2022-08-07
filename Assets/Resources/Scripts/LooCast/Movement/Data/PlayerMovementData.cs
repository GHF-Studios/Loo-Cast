using UnityEngine;

namespace LooCast.Movement.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "PlayerMovementData", menuName = "Data/Movement/PlayerMovementData", order = 0)]
    public class PlayerMovementData : MovementData
    {
        public FloatDataReference BaseEnergy;
        public FloatDataReference BaseMaxEnergy;
        public FloatDataReference BaseEnergyConsumption;
        public FloatDataReference BaseEnergyGeneration;
        public BoolDataReference BaseIsUsingEnergy;
        public BoolDataReference BaseIsEnergyDepleted;
    } 
}
