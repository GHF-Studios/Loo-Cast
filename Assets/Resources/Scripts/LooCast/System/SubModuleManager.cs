using System;
using UnityEngine;

namespace LooCast.System
{
    using global::LooCast.System.MetaData;
    
    public abstract class SubModuleManager<SubModuleManagerType, SubModuleManagerMetaDataType> : ModuleManager<SubModuleManagerType, SubModuleManagerMetaDataType>, ISubModuleManager
        where SubModuleManagerType : SubModuleManager<SubModuleManagerType, SubModuleManagerMetaDataType>, new()
        where SubModuleManagerMetaDataType : SubModuleManagerMetaData, new()
    {
        #region Properties
        public IModuleManager ParentModuleManager { get; private set; }
        #endregion

        #region Overrides
        protected override void PreConstruct()
        {
            base.PreConstruct();

            ParentModuleManager = (IModuleManager)ParentManager;
        }
        #endregion
    }
}