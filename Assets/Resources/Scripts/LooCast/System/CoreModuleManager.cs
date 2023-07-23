using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;

    public abstract class CoreModuleManager : Manager, ICoreModuleManager
    {
        #region Properties
        public string CoreModuleManagerName => ManagerName;

        MainManager IChild<MainManager>.Parent => MainManager.Instance;

        IEnumerable<IModuleManager> IParent<IModuleManager>.Children => ModuleManagerChildren;
        public IEnumerable<IModuleManager> ModuleManagerChildren => moduleManagerChildrenList;
        #endregion

        #region Fields
        protected List<IModuleManager> moduleManagerChildrenList;
        #endregion
        
        #region Constructors
        protected CoreModuleManager() : base()
        {
            moduleManagerChildrenList = new List<IModuleManager>();

            RegisterEarlyPreInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyPreInitialize();
                    }
                });
            RegisterPreInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnPreInitialize();
                    }
                });
            RegisterLatePreInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLatePreInitialize();
                    }
                });
            RegisterEarlyInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyInitialize();
                    }
                });
            RegisterInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnInitialize();
                    }
                });
            RegisterLateInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLateInitialize();
                    }
                });
            RegisterEarlyPostInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyPostInitialize();
                    }
                });
            RegisterPostInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnPostInitialize();
                    }
                });
            RegisterLatePostInitializationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLatePostInitialize();
                    }
                });

            RegisterEarlyPreTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyPreTerminate();
                    }
                });
            RegisterPreTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnPreTerminate();
                    }
                });
            RegisterLatePreTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLatePreTerminate();
                    }
                });
            RegisterEarlyTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyTerminate();
                    }
                });
            RegisterTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnTerminate();
                    }
                });
            RegisterLateTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLateTerminate();
                    }
                });
            RegisterEarlyPostTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnEarlyPostTerminate();
                    }
                });
            RegisterPostTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnPostTerminate();
                    }
                });
            RegisterLatePostTerminationAction(() =>
                {
                    foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
                    {
                        moduleManagerChild.OnLatePostTerminate();
                    }
                });
        }
        #endregion
    }
}