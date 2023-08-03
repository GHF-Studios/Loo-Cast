using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;

    public abstract class CoreModuleManager : Manager, IChild<MainManager>, IParent<ModuleManager>
    {
        #region Properties
        public string CoreModuleManagerName => ManagerName;

        MainManager IChild<MainManager>.Parent => MainManager.Instance;

        IEnumerable<ModuleManager> IParent<ModuleManager>.Children => ModuleManagerChildren;
        public IEnumerable<ModuleManager> ModuleManagerChildren => moduleManagerChildrenList;
        #endregion

        #region Fields
        protected List<ModuleManager> moduleManagerChildrenList;
        #endregion
        
        #region Constructors
        protected CoreModuleManager() : base()
        {
            moduleManagerChildrenList = new List<ModuleManager>();

            RegisterEarlyPreInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyPreInitialize();
                    }
                });
            RegisterPreInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnPreInitialize();
                    }
                });
            RegisterLatePreInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLatePreInitialize();
                    }
                });
            RegisterEarlyInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyInitialize();
                    }
                });
            RegisterInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnInitialize();
                    }
                });
            RegisterLateInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLateInitialize();
                    }
                });
            RegisterEarlyPostInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyPostInitialize();
                    }
                });
            RegisterPostInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnPostInitialize();
                    }
                });
            RegisterLatePostInitializationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLatePostInitialize();
                    }
                });

            RegisterEarlyPreTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyPreTerminate();
                    }
                });
            RegisterPreTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnPreTerminate();
                    }
                });
            RegisterLatePreTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLatePreTerminate();
                    }
                });
            RegisterEarlyTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyTerminate();
                    }
                });
            RegisterTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnTerminate();
                    }
                });
            RegisterLateTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLateTerminate();
                    }
                });
            RegisterEarlyPostTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyPostTerminate();
                    }
                });
            RegisterPostTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnPostTerminate();
                    }
                });
            RegisterLatePostTerminationAction(() =>
                {
                    foreach (ModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLatePostTerminate();
                    }
                });
        }
        #endregion
    }
}