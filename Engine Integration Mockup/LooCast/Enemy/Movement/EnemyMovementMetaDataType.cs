using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Movement
{
    [MetaDataType]
    public abstract class EnemyMovementMetaDataType<T> : MovementMetaDataType<T> where T : EnemyMovementMetaData
    {
        
    }
}