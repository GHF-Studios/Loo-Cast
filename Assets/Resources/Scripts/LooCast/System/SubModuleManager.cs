using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    
    public abstract class SubModuleManager : Manager, IChild<ModuleManager>, IChild<SubModuleManager>, IParent<SubModuleManager>
    {
        #region Properties
        public string SubModuleManagerName => ManagerName;

        ModuleManager IChild<ModuleManager>.Parent => ModuleManagerParent;
        public ModuleManager ModuleManagerParent { get; private set; }

        SubModuleManager IChild<SubModuleManager>.Parent => SubModuleManagerParent;
        public SubModuleManager SubModuleManagerParent { get; private set; }

        IEnumerable<SubModuleManager> IParent<SubModuleManager>.Children => throw new NotImplementedException();
        public IEnumerable<SubModuleManager> SubModuleManagerChildren => subModuleManagerChildrenList;
        #endregion

        #region Fields
        protected List<SubModuleManager> subModuleManagerChildrenList;
        #endregion

        #region Constructors
        protected SubModuleManager() : base()
        {
            subModuleManagerChildrenList = new List<SubModuleManager>();

            RegisterEarlyPreInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyPreInitialize();
                    }
                });
            RegisterPreInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnPreInitialize();
                    }
                });
            RegisterLatePreInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLatePreInitialize();
                    }
                });
            RegisterEarlyInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyInitialize();
                    }
                });
            RegisterInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnInitialize();
                    }
                });
            RegisterLateInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLateInitialize();
                    }
                });
            RegisterEarlyPostInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyPostInitialize();
                    }
                });
            RegisterPostInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnPostInitialize();
                    }
                });
            RegisterLatePostInitializationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLatePostInitialize();
                    }
                });

            RegisterEarlyPreTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyPreTerminate();
                    }
                });
            RegisterPreTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnPreTerminate();
                    }
                });
            RegisterLatePreTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLatePreTerminate();
                    }
                });
            RegisterEarlyTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyTerminate();
                    }
                });
            RegisterTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnTerminate();
                    }
                });
            RegisterLateTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLateTerminate();
                    }
                });
            RegisterEarlyPostTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnEarlyPostTerminate();
                    }
                });
            RegisterPostTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnPostTerminate();
                    }
                });
            RegisterLatePostTerminationAction(() =>
                {
                    foreach (SubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
                    {
                        subModuleManagerChild.OnLatePostTerminate();
                    }
                });
        }
        #endregion
    }
}