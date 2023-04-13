using System;

namespace LooCast.System
{
    public interface IModuleManager : IManager
    {
        #region Properties
        public ISubModuleManager[] SubModuleManagers { get; }
        public ICoreModuleManager ParentCoreModuleManager { get; }
        #endregion

        #region Methods
        #endregion
    }
}
