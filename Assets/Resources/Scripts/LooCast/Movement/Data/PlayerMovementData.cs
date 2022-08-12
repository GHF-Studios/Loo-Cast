using UnityEngine;

namespace LooCast.Movement.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "PlayerMovementData", menuName = "Data/Movement/PlayerMovementData", order = 0)]
    public class PlayerMovementData : ScriptableObject
    {
        public FloatDataReference BaseSpeed;
        public FloatDataReference BaseMaxEnergy;
        public FloatDataReference BaseEnergyConsumption;
        public FloatDataReference BaseEnergyGeneration;
        public BoolDataReference BaseIsUsingEnergy;
        public BoolDataReference BaseIsEnergyDepleted;
    } 
}
