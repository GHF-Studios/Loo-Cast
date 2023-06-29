using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
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
        private List<ISubModuleManager> subModuleManagerChildrenList;
        #endregion

        #region Constructors
        protected ModuleManager(string moduleManagerName, ICoreModuleManager coreModuleManagerParent) : base(moduleManagerName, coreModuleManagerParent, System.ManagerMonoBehaviour.Create(moduleManagerName))
        {
            subModuleManagerChildrenList = new List<ISubModuleManager>();

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
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.EarlyPreInitialize();
            }
        }

        private void OnPreInitialize()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.PreInitialize();
            }
        }

        private void OnLatePreInitialize()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.LatePreInitialize();
            }
        }

        private void OnEarlyInitialize()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.EarlyInitialize();
            }
        }

        private void OnInitialize()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.Initialize();
            }
        }

        private void OnLateInitialize()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.LateInitialize();
            }
        }

        private void OnEarlyPostInitialize()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.EarlyPostInitialize();
            }
        }

        private void OnPostInitialize()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.PostInitialize();
            }
        }

        private void OnLatePostInitialize()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.LatePostInitialize();
            }
        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.EarlyPreTerminate();
            }
        }

        private void OnPreTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.PreTerminate();
            }
        }

        private void OnLatePreTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.LatePreTerminate();
            }
        }

        private void OnEarlyTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.EarlyTerminate();
            }
        }

        private void OnTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.Terminate();
            }
        }

        private void OnLateTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.LateTerminate();
            }
        }

        private void OnEarlyPostTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.EarlyPostTerminate();
            }
        }

        private void OnPostTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.PostTerminate();
            }
        }

        private void OnLatePostTerminate()
        {
            foreach (ISubModuleManager subModuleManagerChild in subModuleManagerChildrenList)
            {
                subModuleManagerChild.LatePostTerminate();
            }
        }
        #endregion

        #endregion

        #region Methods
        public bool TryAddChildSubModuleManager(ISubModuleManager childSubModuleManager)
        {
            if (ContainsChildSubModuleManager(childSubModuleManager.SubModuleManagerName))
            {
                return false;
            }
            else
            {
                AddChildSubModuleManager(childSubModuleManager);
                return true;
            }
        }
        public void AddChildSubModuleManager(ISubModuleManager childSubModuleManager)
        {
            subModuleManagerChildrenList.Add(childSubModuleManager);
        }

        public bool TryRemoveChildSubModuleManager(ISubModuleManager childSubModuleManager)
        {
            if (!ContainsChildSubModuleManager(childSubModuleManager))
            {
                return false;
            }
            else
            {
                RemoveChildSubModuleManager(childSubModuleManager);
                return true;
            }
        }
        public void RemoveChildSubModuleManager(ISubModuleManager childSubModuleManager)
        {
            subModuleManagerChildrenList.Remove(childSubModuleManager);
        }

        public bool TryGetChildSubModuleManager(string childSubModuleManagerName, out ISubModuleManager childSubModuleManager)
        {
            if (!ContainsChildSubModuleManager(childSubModuleManagerName))
            {
                childSubModuleManager = null;
                return false;
            }
            else
            {
                childSubModuleManager = GetChildSubModuleManager(childSubModuleManagerName);
                return true;
            }
        }
        public ISubModuleManager GetChildSubModuleManager(string childSubModuleManagerName)
        {
            return subModuleManagerChildrenList.Find((subModuleManagerChild) => { return subModuleManagerChild.SubModuleManagerName == childSubModuleManagerName; });
        }

        public bool ContainsChildSubModuleManager(string childSubModuleManagerName)
        {
            return subModuleManagerChildrenList.Exists((subModuleManagerChild) => { return subModuleManagerChild.SubModuleManagerName == childSubModuleManagerName; });
        }

        public bool ContainsChildSubModuleManager(ISubModuleManager childSubModuleManager)
        {
            return subModuleManagerChildrenList.Contains(childSubModuleManager);
        }

        public void ClearChildSubModuleManagers()
        {
            subModuleManagerChildrenList.Clear();
        }
        #endregion
    }
}