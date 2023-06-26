using System;

namespace LooCast.System
{
    public interface ICoreModuleManager : IManager, IChild<MainManager>, IParent<IModuleManager>
    {
        #region Properties
        #endregion

        #region Methods
        bool TryAddChildModuleManager(IModuleManager childModuleManager);
        void AddChildModuleManager(IModuleManager childModuleManager);

        bool TryRemoveChildModuleManager(IModuleManager childModuleManager);
        void RemoveChildModuleManager(IModuleManager childModuleManager);

        bool TryGetChildModuleManager(string childModuleManagerName, out IModuleManager childModuleManager);
        IModuleManager GetChildModuleManager(string childModuleManagerName);

        bool ContainsChildModuleManager(string childModuleManagerName);
        bool ContainsChildModuleManager(IModuleManager childModuleManager);

        void ClearChildModuleManagers();
        #endregion
    }
}
