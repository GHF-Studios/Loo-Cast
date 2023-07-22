using System;

namespace LooCast.System
{
    public interface ISubModuleManager : IManager, IChild<IModuleManager>, IChild<ISubModuleManager>, IParent<ISubModuleManager>
    {
        #region Properties
        string SubModuleManagerName { get; }
        #endregion
    }
}
