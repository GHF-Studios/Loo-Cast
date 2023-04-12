using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Health
{
    [Module]
    public class HealthModule : Module
    {
        #region Sub Modules

        #endregion

        #region Entity Types
        #endregion

        #region Component Types
        [PersistableComponentTypeInstance]
        public static HealthPersistableComponentType<HealthPersistableComponent> HealthPersistableComponentType;
        #endregion

        #region MetaData Types
        [MetaDataTypeInstance]
        public static HealthMetaDataType<HealthMetaData> HealthMetaDataType;
        #endregion

        #region Data Types
        [PersistableDataTypeInstance]
        public static HealthPersistableDataType<HealthPersistableData> HealthPersistableDataType;
        #endregion

        #region Logic Types
        [LogicTypeInstance]
        public static HealthLogicType<HealthLogic> HealthLogicType;
        #endregion
    } 
}
