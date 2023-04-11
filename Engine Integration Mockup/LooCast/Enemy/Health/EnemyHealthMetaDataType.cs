using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Health
{
    [MetaDataType]
    public abstract class EnemyHealthMetaDataType<T> : HealthMetaDataType<T> where T : EnemyHealthMetaData
    {
        
    }
}