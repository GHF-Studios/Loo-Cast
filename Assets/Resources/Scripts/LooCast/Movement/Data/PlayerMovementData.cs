using UnityEngine;

namespace LooCast.Movement.Data
{
    using LooCast.Data;

    public abstract class PlayerMovementData : MovementData
    {
        public FloatReference BaseEnergy;
        public FloatReference BaseMaxEnergy;
        public FloatReference BaseEnergyConsumption;
        public FloatReference BaseEnergyGeneration;
        public BoolReference IsUsingEnergy;
        public BoolReference IsEnergyDepleted;
    } 
}
