using System;

namespace LooCast.System
{
    public interface IModuleManager : IManager, IChild<ICoreModuleManager>, IParent<ISubModuleManager>
    {
        #region Properties
        string ModuleManagerName { get; }
        #endregion
    }
}
