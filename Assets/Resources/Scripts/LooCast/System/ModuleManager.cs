using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;

    public abstract class ModuleManager : Manager, IModuleManager
    {
        #region Properties
        public string ModuleManagerName => ManagerName;

        ICoreModuleManager IChild<ICoreModuleManager>.Parent => CoreModuleManagerParent;
        public ICoreModuleManager CoreModuleManagerParent { get; private set; }

        IEnumerable<ISubModuleManager> IParent<ISubModuleManager>.Children => SubModuleManagerChildren;
        public IEnumerable<ISubModuleManager> SubModuleManagerChildren => subModuleManagerChildrenList;
        #endregion

        #region Fields
        protected List<ISubModuleManager> subModuleManagerChildrenList;
        #endregion

        #region Constructors
        protected ModuleManager() : base()
        {
            subModuleManagerChildrenList = new List<ISubModuleManager>();

            RegisterEarlyPreInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyPreInitialize();
                    }
                });
            RegisterPreInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnPreInitialize();
                    }
                });
            RegisterLatePreInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLatePreInitialize();
                    }
                });
            RegisterEarlyInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyInitialize();
                    }
                });
            RegisterInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnInitialize();
                    }
                });
            RegisterLateInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLateInitialize();
                    }
                });
            RegisterEarlyPostInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyPostInitialize();
                    }
                });
            RegisterPostInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnPostInitialize();
                    }
                });
            RegisterLatePostInitializationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLatePostInitialize();
                    }
                });

            RegisterEarlyPreTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyPreTerminate();
                    }
                });
            RegisterPreTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnPreTerminate();
                    }
                });
            RegisterLatePreTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLatePreTerminate();
                    }
                });
            RegisterEarlyTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyTerminate();
                    }
                });
            RegisterTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnTerminate();
                    }
                });
            RegisterLateTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLateTerminate();
                    }
                });
            RegisterEarlyPostTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyPostTerminate();
                    }
                });
            RegisterPostTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnPostTerminate();
                    }
                });
            RegisterLatePostTerminationAction(() =>
                {
                    foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLatePostTerminate();
                    }
                });
        }
        #endregion
    }
}