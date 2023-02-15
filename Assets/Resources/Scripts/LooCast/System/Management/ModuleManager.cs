using System;
using UnityEngine;
using LooCast;

namespace LooCast.System.Management
{ 
    public abstract class ModuleManager : Manager
    {
        #region Properties
        public SubModuleManager[] SubModuleManagers { get; private set; }
        #endregion

        #region Callbacks

        #region Initialization Phases
        private void OnEarlyPreInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyPreInitialize();
            }
        }

        private void OnPreInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.PreInitialize();
            }
        }

        private void OnLatePreInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LatePreInitialize();
            }
        }

        private void OnEarlyInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyInitialize();
            }
        }

        private void OnInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.Initialize();
            }
        }

        private void OnLateInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LateInitialize();
            }
        }

        private void OnEarlyPostInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyPostInitalize();
            }
        }

        private void OnPostInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.PostInitialize();
            }
        }

        private void OnLatePostInitialize()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LatePostInitialize();
            }
        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyPreTerminate();
            }
        }

        private void OnPreTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.PreTerminate();
            }
        }

        private void OnLatePreTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LatePreTerminate();
            }
        }

        private void OnEarlyTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyTerminate();
            }
        }

        private void OnTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.Terminate();
            }
        }

        private void OnLateTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LateTerminate();
            }
        }

        private void OnEarlyPostTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyPostTerminate();
            }
        }

        private void OnPostTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.PostTerminate();
            }
        }

        private void OnLatePostTerminate()
        {
            foreach (SubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LatePostTerminate();
            }
        }
        #endregion

        #endregion

        #region Methods
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            SubModuleManagers = GetSubModuleManagers();

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

        /// <summary>
        /// Returns the sub-module managers in the order they should be initialized.
        /// </summary>
        protected virtual SubModuleManager[] GetSubModuleManagers()
        {
            return new SubModuleManager[0];
        }
        #endregion
    }
}