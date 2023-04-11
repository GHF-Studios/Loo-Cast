using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Movement
{
    [LogicType]
    public abstract class EnemyMovementLogicType<T> : MovementLogicType<T> where T : EnemyMovementLogic
    {
        
    }
}