using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Health
{
    [PersistableComponent]
    public abstract class EnemyHealthPersistableComponent : HealthPersistableComponent
    {
        [MetaDataInstance]
        public EnemyHealthMetaData MetaData;

        [PersistableDataInstance]
        public EnemyHealthPersistableData PersistableData;

        [LogicInstance]
        public EnemyHealthLogic Logic;
    } 
}
