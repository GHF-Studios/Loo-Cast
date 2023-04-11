using System;
using UnityEngine;
using LooCast.Core;
using LooCast.Enemy.Health;
using LooCast.Enemy.Movement;

namespace LooCast.Enemy
{
    [PersistableEntity]
    public abstract class EnemyPersistableEntity : PersistableEntity
    {
        [PersistableComponentInstance]
        public EnemyHealthPersistableComponent HealthPersistableComponent;

        [PersistableComponentInstance]
        public EnemyMovementPersistableComponent MovementPersistableComponent;
    } 
}
