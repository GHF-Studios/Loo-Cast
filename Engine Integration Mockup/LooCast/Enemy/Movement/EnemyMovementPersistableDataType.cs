using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Movement
{
    [PersistableDataType]
    public abstract class EnemyMovementPersistableDataType<T> : MovementPersistableDataType<T> where T : EnemyMovementPersistableData
    {
        
    }
}