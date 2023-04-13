using System;

namespace LooCast.System
{
    public interface ICoreModuleManager : IModuleManager
    {
        #region Properties
        public IModuleManager[] ModuleManagers { get; }
        public MainManager ParentMainManager { get; }
        #endregion

        #region Methods
        #endregion
    }
}
