using System;
using UnityEngine;
using LooCast.Core;
using LooCast.Enemy.Health;
using LooCast.Enemy.Movement;

namespace LooCast.Enemy
{
    [Module]
    public class EnemyModule : Module
    {
        #region Sub Modules
        [SubModuleInstance]
        public static EnemyHealthSubModule EnemyHealthSubModule;

        [SubModuleInstance]
        public static EnemyMovementSubModule EnemyMovementSubModule;
        #endregion

        #region Entity Types
        [PersistableEntityTypeInstance]
        public static EnemyPersistableEntityType<EnemyPersistableEntity> EnemyPersistableEntityType;
        #endregion

        #region Component Types
        #endregion

        #region MetaData Types
        #endregion

        #region Data Types
        #endregion

        #region Logic Types
        #endregion
    } 
}
