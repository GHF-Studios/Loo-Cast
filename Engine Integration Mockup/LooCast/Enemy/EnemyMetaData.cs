using System;
using UnityEngine;

namespace LooCast.Enemy
{
    [MetaData]
    public abstract class EnemyMetaData : MetaData
    {
        [MetaDataInstance]
        public EnemyMovementMetaData MovementMetaData;

        [MetaDataInstance]
        public EnemyHealthMetaData HealthMetaData;
    }
}