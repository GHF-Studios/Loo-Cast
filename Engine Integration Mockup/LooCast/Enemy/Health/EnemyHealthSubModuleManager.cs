using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Health
{
    [SubModuleManager]
    public class EnemyHealthSubModuleManager : SubModuleManager
    {
        #region Entity Types
        #endregion

        #region Component Types
        [PersistableComponentTypeInstance]
        public static EnemyHealthPersistableComponentType<EnemyHealthPersistableComponent> EnemyHealthPersistableComponentType;
        #endregion

        #region MetaData Types
        [MetaDataTypeInstance]
        public static EnemyHealthMetaDataType<EnemyHealthMetaData> EnemyHealthMetaDataType;
        #endregion

        #region Data Types
        [PersistableDataTypeInstance]
        public static EnemyHealthPersistableDataType<EnemyHealthPersistableData> EnemyHealthPersistableDataType;
        #endregion

        #region Logic Types
        [LogicTypeInstance]
        public static EnemyHealthLogicType<EnemyHealthLogic> EnemyHealthLogicType;
        #endregion
    } 
}
