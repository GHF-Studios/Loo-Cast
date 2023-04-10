using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy
{
    [ModuleManager]
    public class EnemyModuleManager : ModuleManager
    {
        #region Types

        #region MetaData Types
        [MetaDataTypeInstance]
        public static EnemyMetaDataType EnemyMetaDataType;
        #endregion

        #region Persistable Entity Types
        [PersistableEntityTypeInstance]
        public static EnemyPersistableEntityType EnemyPersistableEntityType;
        #endregion

        #region Non-Persistable Entity Types
        #endregion

        #region Persistable Data Types
        [PersistableDataTypeInstance]
        public static EnemyPersistableDataType EnemyPersistableDataType;

        [PersistableDataTypeInstance]
        public static EnemyHealthPersistableDataType EnemyHealthPersistableDataType;

        [PersistableDataTypeInstance]
        public static EnemyMovementPersistableDataType EnemyMovementPersistableDataType;
        #endregion

        #region Non-Persistable Data Types
        #endregion

        #region Logic Types
        [LogicTypeInstance]
        public static EnemyLogicType EnemyLogicType;

        [LogicTypeInstance]
        public static EnemyHealthLogicType EnemyHealthLogicType;

        [LogicTypeInstance]
        public static EnemyMovementLogicType EnemyMovementLogicType;
        #endregion

        #endregion
    } 
}
