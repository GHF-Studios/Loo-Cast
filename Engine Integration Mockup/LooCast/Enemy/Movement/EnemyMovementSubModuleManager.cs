using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Movement
{
    [SubModuleManager]
    public class EnemyMovementSubModuleManager : SubModuleManager
    {
        #region Entity Types
        #endregion

        #region Component Types 
        [PersistableComponentTypeInstance]
        public static EnemyMovementPersistableComponentType<EnemyMovementPersistableComponent> EnemyMovementPersistableComponentType;
        #endregion

        #region MetaData Types
        [MetaDataTypeInstance]
        public static EnemyMovementMetaDataType<EnemyMovementMetaData> EnemyMovementMetaDataType;
        #endregion

        #region Data Types
        [PersistableDataTypeInstance]
        public static EnemyMovementPersistableDataType<EnemyMovementPersistableData> EnemyMovementPersistableDataType;
        #endregion

        #region Logic Types
        [LogicTypeInstance]
        public static EnemyMovementLogicType<EnemyMovementLogic> EnemyMovementLogicType;
        #endregion
    } 
}
