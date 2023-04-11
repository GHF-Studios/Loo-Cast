using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Health
{
    [PersistableDataType]
    public abstract class EnemyHealthPersistableDataType<T> : HealthPersistableDataType<T> where T : EnemyHealthPersistableData
    {
        
    }
}