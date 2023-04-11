using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Health
{
    [LogicType]
    public abstract class EnemyHealthLogicType<T> : HealthLogicType<T> where T : EnemyHealthLogic
    {
        
    }
}