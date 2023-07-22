using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Serialization;
    using LooCast.System.ECS;
    
    public abstract class SubModuleManager : Manager, ISubModuleManager
    {
        #region Properties
        public string SubModuleManagerName => ManagerName;

        IModuleManager IChild<IModuleManager>.Parent => ModuleManagerParent;
        public IModuleManager ModuleManagerParent { get; private set; }

        ISubModuleManager IChild<ISubModuleManager>.Parent => SubModuleManagerParent;
        public ISubModuleManager SubModuleManagerParent { get; private set; }

        IEnumerable<ISubModuleManager> IParent<ISubModuleManager>.Children => throw new NotImplementedException();
        public IEnumerable<ISubModuleManager> SubModuleManagerChildren => subModuleManagerChildrenList;
        #endregion

        #region Fields
        private List<ISubModuleManager> subModuleManagerChildrenList;
        #endregion

        #region Constructors
        protected SubModuleManager() : base()
        {
            RegisterPreSetupAction(() =>
            {
                subModuleManagerChildrenList = new List<ISubModuleManager>();
            });

            RegisterPostSetupAction(() =>
            {
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

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                    subModuleManager.OnSetup();
                    subModuleManager.OnPostSetup();
                }
            });
        }
        #endregion

        #region Methods
        protected void AddChildSubModuleManager(ISubModuleManager childSubModuleManager)
        {
            if (childSubModuleManager is null)
            {
                throw new ArgumentNullException(nameof(childSubModuleManager));
            }
            if (IsPreInitializing || IsPreInitialized)
            {
                throw new InvalidOperationException("Child sub module managers have to be added before pre-initialization!");
            }
            
            subModuleManagerChildrenList.Add(childSubModuleManager);
        }
        #endregion
    }
}