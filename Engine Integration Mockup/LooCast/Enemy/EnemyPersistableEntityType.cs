using System;
using UnityEngine;
using LooCast.Core;
using LooCast.Enemy.Health;
using LooCast.Enemy.Movement;

namespace LooCast.Enemy
{
    [PersistableEntityType]
    public abstract class EnemyPersistableEntityType<T> : PersistableEntityType<T> where T : EnemyPersistableEntity
    {
        [PersistableComponentTypeInstance]
        public EnemyHealthPersistableComponentType HealthPersistableComponentType;

        [PersistableComponentTypeInstance]
        public EnemyMovementPersistableComponentType MovementPersistableComponentType;
    }
}