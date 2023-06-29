using System;

namespace LooCast.System
{
    public interface ISubModuleManager : IManager, IChild<IModuleManager>, IChild<ISubModuleManager>, IParent<ISubModuleManager>
    {
        #region Properties
        string SubModuleManagerName { get; }
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
