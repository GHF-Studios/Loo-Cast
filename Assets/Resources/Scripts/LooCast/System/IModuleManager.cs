using System;

namespace LooCast.System
{
    public interface IModuleManager : IManager, IChild<ICoreModuleManager>, IParent<ISubModuleManager>
    {
        #region Properties
        string ModuleManagerName { get; }
        #endregion

        #region Methods
        bool TryAddChildSubModuleManager(ISubModuleManager childSubModuleManager);
        void AddChildSubModuleManager(ISubModuleManager childSubModuleManager);

        bool TryRemoveChildSubModuleManager(ISubModuleManager childSubModuleManager);
        void RemoveChildSubModuleManager(ISubModuleManager childSubModuleManager);

        bool TryGetChildSubModuleManager(string childSubModuleManagerName, out ISubModuleManager childSubModuleManager);
        ISubModuleManager GetChildSubModuleManager(string childSubModuleManagerName);

        bool ContainsChildSubModuleManager(string childSubModuleManagerName);
        bool ContainsChildSubModuleManager(ISubModuleManager childSubModuleManager);

        void ClearChildSubModuleManagers();
        #endregion
    }
}
