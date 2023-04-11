using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Movement
{
    [ModuleManager]
    public class MovementModuleManager : ModuleManager
    {
        #region Sub Module Managers

        #endregion

        #region Entity Types
        #endregion

        #region Component Types
        [PersistableComponentTypeInstance]
        public static MovementPersistableComponentType<MovementPersistableComponent> MovementPersistableComponentType;
        #endregion

        #region MetaData Types
        [MetaDataTypeInstance]
        public static MovementMetaDataType<MovementMetaData> MovementMetaDataType;
        #endregion

        #region Data Types
        [PersistableDataTypeInstance]
        public static MovementPersistableDataType<MovementPersistableData> MovementPersistableDataType;
        #endregion

        #region Logic Types
        [LogicTypeInstance]
        public static MovementLogicType<MovementLogic> MovementLogicType;
        #endregion
    } 
}
