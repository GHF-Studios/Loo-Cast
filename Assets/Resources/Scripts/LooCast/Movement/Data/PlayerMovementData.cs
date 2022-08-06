using UnityEngine;

namespace LooCast.Movement.Data
{
    using LooCast.Data;

    public abstract class PlayerMovementData : MovementData
    {
        public FloatDataReference BaseEnergy;
        public FloatDataReference BaseMaxEnergy;
        public FloatDataReference BaseEnergyConsumption;
        public FloatDataReference BaseEnergyGeneration;
        public BoolDataReference IsUsingEnergy;
        public BoolDataReference IsEnergyDepleted;
    } 
}
