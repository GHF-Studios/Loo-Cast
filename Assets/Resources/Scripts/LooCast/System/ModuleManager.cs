﻿using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;

    public abstract class ModuleManager : Manager, IChild<CoreModuleManager>, IParent<SubModuleManager>
    {
        #region Properties
        public string ModuleManagerName => ManagerName;

        CoreModuleManager IChild<CoreModuleManager>.Parent => CoreModuleManagerParent;
        public CoreModuleManager CoreModuleManagerParent { get; private set; }

        IEnumerable<SubModuleManager> IParent<SubModuleManager>.Children => SubModuleManagerChildren;
        public IEnumerable<SubModuleManager> SubModuleManagerChildren => subModuleManagerChildrenList;
        #endregion

        #region Fields
        protected List<SubModuleManager> subModuleManagerChildrenList;
        #endregion

        #region Constructors
        protected ModuleManager() : base()
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