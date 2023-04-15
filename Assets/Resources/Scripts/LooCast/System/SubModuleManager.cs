using System;
using UnityEngine;

namespace LooCast.System
{
    public abstract class SubModuleManager<SubModuleManagerType> : ModuleManager<SubModuleManagerType>, ISubModuleManager
        where SubModuleManagerType : SubModuleManager<SubModuleManagerType>, new()
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