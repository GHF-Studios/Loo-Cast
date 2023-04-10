using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy
{
    [PersistableEntity]
    public abstract class Enemy
    {
        #region Components
        [EntityComponentInstance]
        public Entity Entity;

        [MetaDataComponentInstance]
        public EnemyMetaData EnemyMetaData;
        [DataComponentInstance]
        public EnemyData EnemyData;

        [LogicComponentInstance]
        public EnemyMovement Movement;
        [LogicComponentInstance]
        public EnemyHealth Health;
        [LogicComponentInstance]
        public ParticleSystem ParticleSystem;
        #endregion
    } 
}
