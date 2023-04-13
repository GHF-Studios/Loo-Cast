using System;

namespace LooCast.System
{
    public interface ISubModuleManager : IModuleManager
    {
        #region Properties
        public IModuleManager ParentModuleManager { get; }
        #endregion

        #region Methods
        #endregion
    }
}
