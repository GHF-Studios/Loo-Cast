using UnityEngine;

namespace LooCast.Movement.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyMovementData", menuName = "Data/Movement/EnemyMovementData", order = 0)]
    public class EnemyMovementData : ScriptableObject
    {
        public FloatDataReference BaseSpeed;
    } 
}
