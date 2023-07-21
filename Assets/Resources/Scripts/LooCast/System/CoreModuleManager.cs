using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    public abstract class CoreModuleManager : Manager, ICoreModuleManager
    {
        #region Properties
        public string CoreModuleManagerName => ManagerName;

        MainManager IChild<MainManager>.Parent => MainManager.Instance;

        IEnumerable<IModuleManager> IParent<IModuleManager>.Children => ModuleManagerChildren;
        public IEnumerable<IModuleManager> ModuleManagerChildren => moduleManagerChildrenList;
        #endregion

        #region Fields
        private List<IModuleManager> moduleManagerChildrenList;
        #endregion
        
        #region Constructors
        protected CoreModuleManager(string coreModuleManagerName) : base(coreModuleManagerName, MainManager.Instance)
        {
            RegisterPreSetupAction(() =>
            {
                moduleManagerChildrenList = new List<IModuleManager>();
            });

            RegisterPostSetupAction(() =>
            {
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

                foreach (IModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnPreSetup();
                    moduleManager.OnSetup();
                    moduleManager.OnPostSetup();
                }
            });

        }
        #endregion

        #region Methods
        protected void AddChildModuleManager(IModuleManager childModuleManager)
        {
            if (childModuleManager is null)
            {
                throw new ArgumentNullException(nameof(childModuleManager));
            }
            if (IsPreInitializing || IsPreInitialized)
            {
                throw new InvalidOperationException("Child module managers have to be added before pre-initialization!");
            }
            
            moduleManagerChildrenList.Add(childModuleManager);
        }
        #endregion
    }
}