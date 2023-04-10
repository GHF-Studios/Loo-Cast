using System;
using UnityEngine;

namespace LooCast.Enemy
{
    [PersistableData]
    public abstract class EnemyPersistableData : PersistableData
    {
        [PersistableDataInstance]
        public EnemyMovementPersistentData MovementPersistentData;

        [PersistableDataInstance]
        public EnemyHealthPersistentData HealthPersistentData;
    }
}