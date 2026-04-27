using UnityEngine;

namespace LooCast.Movement.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AllyMovementData", menuName = "Data/Movement/AllyMovementData", order = 0)]
    public class AllyMovementData : ScriptableObject
    {
        public FloatDataReference BaseSpeed;
    } 
}
