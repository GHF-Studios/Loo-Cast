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
        protected CoreModuleManager(string coreModuleManagerName) : base(coreModuleManagerName, MainManager.Instance, System.ManagerMonoBehaviour.Create(coreModuleManagerName))
        {
            moduleManagerChildrenList = new List<IModuleManager>();

            RegisterEarlyPreInitializationAction(OnEarlyPreInitialize);
            RegisterPreInitializationAction(OnPreInitialize);
            RegisterLatePreInitializationAction(OnLatePreInitialize);
            RegisterEarlyInitializationAction(OnEarlyInitialize);
            RegisterInitializationAction(OnInitialize);
            RegisterLateInitializationAction(OnLateInitialize);
            RegisterEarlyPostInitializationAction(OnEarlyPostInitialize);
            RegisterPostInitializationAction(OnPostInitialize);
            RegisterLatePostInitializationAction(OnLatePostInitialize);

            RegisterEarlyPreTerminationAction(OnEarlyPreTerminate);
            RegisterPreTerminationAction(OnPreTerminate);
            RegisterLatePreTerminationAction(OnLatePreTerminate);
            RegisterEarlyTerminationAction(OnEarlyTerminate);
            RegisterTerminationAction(OnTerminate);
            RegisterLateTerminationAction(OnLateTerminate);
            RegisterEarlyPostTerminationAction(OnEarlyPostTerminate);
            RegisterPostTerminationAction(OnPostTerminate);
            RegisterLatePostTerminationAction(OnLatePostTerminate);
        }
        #endregion

        #region Callbacks

        #region Initialization Phases
        private void OnEarlyPreInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.EarlyPreInitialize();
            }
        }

        private void OnPreInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.PreInitialize();
            }
        }

        private void OnLatePreInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.LatePreInitialize();
            }
        }

        private void OnEarlyInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.EarlyInitialize();
            }
        }

        private void OnInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.Initialize();
            }
        }

        private void OnLateInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.LateInitialize();
            }
        }

        private void OnEarlyPostInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.EarlyPostInitialize();
            }
        }

        private void OnPostInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.PostInitialize();
            }
        }

        private void OnLatePostInitialize()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.LatePostInitialize();
            }
        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.EarlyPreTerminate();
            }
        }

        private void OnPreTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.PreTerminate();
            }
        }

        private void OnLatePreTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.LatePreTerminate();
            }
        }

        private void OnEarlyTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.EarlyTerminate();
            }
        }

        private void OnTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.Terminate();
            }
        }

        private void OnLateTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.LateTerminate();
            }
        }

        private void OnEarlyPostTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.EarlyPostTerminate();
            }
        }

        private void OnPostTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.PostTerminate();
            }
        }

        private void OnLatePostTerminate()
        {
            foreach (IModuleManager moduleManagerChild in moduleManagerChildrenList)
            {
                moduleManagerChild.LatePostTerminate();
            }
        }
        #endregion

        #endregion

        #region Methods
        public bool TryAddChildModuleManager(IModuleManager childModuleManager)
        {
            if (ContainsChildModuleManager(childModuleManager.ModuleManagerName))
            {
                return false;
            }
            else
            {
                AddChildModuleManager(childModuleManager);
                return true;
            }
        }
        public void AddChildModuleManager(IModuleManager childModuleManager)
        {
            moduleManagerChildrenList.Add(childModuleManager);
        }

        public bool TryRemoveChildModuleManager(IModuleManager childModuleManager)
        {
            if (!ContainsChildModuleManager(childModuleManager))
            {
                return false;
            }
            else
            {
                RemoveChildModuleManager(childModuleManager);
                return true;
            }
        }
        public void RemoveChildModuleManager(IModuleManager childModuleManager)
        {
            moduleManagerChildrenList.Remove(childModuleManager);
        }

        public bool TryGetChildModuleManager(string childModuleManagerName, out IModuleManager childModuleManager)
        {
            if (!ContainsChildModuleManager(childModuleManagerName))
            {
                childModuleManager = null;
                return false;
            }
            else
            {
                childModuleManager = GetChildModuleManager(childModuleManagerName);
                return true;
            }
        }
        public IModuleManager GetChildModuleManager(string childModuleManagerName)
        {
            return moduleManagerChildrenList.Find((moduleManagerChild) => { return moduleManagerChild.ModuleManagerName == childModuleManagerName; });
        }

        public bool ContainsChildModuleManager(string childModuleManagerName)
        {
            return moduleManagerChildrenList.Exists((moduleManagerChild) => { return moduleManagerChild.ModuleManagerName == childModuleManagerName; });
        }

        public bool ContainsChildModuleManager(IModuleManager childModuleManager)
        {
            return moduleManagerChildrenList.Contains(childModuleManager);
        }

        public void ClearChildModuleManagers()
        {
            moduleManagerChildrenList.Clear();
        }
        #endregion
    }
}