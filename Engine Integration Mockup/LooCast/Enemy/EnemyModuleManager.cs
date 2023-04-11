using System;
using UnityEngine;
using LooCast.Core;
using LooCast.Enemy.Health;
using LooCast.Enemy.Movement;

namespace LooCast.Enemy
{
    [ModuleManager]
    public class EnemyModuleManager : ModuleManager
    {
        #region Sub Module Managers
        [SubModuleManagerInstance]
        public static EnemyHealthSubModuleManager EnemyHealthSubModuleManager;

        [SubModuleManagerInstance]
        public static EnemyMovementSubModuleManager EnemyMovementSubModuleManager;
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
