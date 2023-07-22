using System;

namespace LooCast.System
{
    public interface ICoreModuleManager : IManager, IChild<MainManager>, IParent<IModuleManager>
    {
        #region Properties
        string CoreModuleManagerName { get; }
        #endregion
    }
}
