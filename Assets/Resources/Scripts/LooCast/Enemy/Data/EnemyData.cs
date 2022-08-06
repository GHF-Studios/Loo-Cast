using UnityEngine;

namespace LooCast.Enemy.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyData", menuName = "Data/Enemy/EnemyData", order = 0)]
    public class EnemyData : ScriptableObject
    {
        public FloatDataReference ContactDamage;
    }
}
