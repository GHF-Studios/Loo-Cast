using System;
using UnityEngine;
using LooCast.Core;
using LooCast.Movement;
using LooCast.Health;

namespace LooCast.Enemy
{
    [Logic]
    public abstract class EnemyLogic : Logic
    {
        [LogicInstance]
        public EnemyMovementLogic MovementLogic;

        [LogicInstance]
        public EnemyHealthLogic HealthLogic;
    }
}