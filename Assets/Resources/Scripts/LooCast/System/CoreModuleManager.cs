using System;
using UnityEngine;

namespace LooCast.System
{
    public abstract class CoreModuleManager : ModuleManager, ICoreModuleManager
    {
        #region Properties
        #endregion

        #region Callbacks

        #region Initialization Phases
        private void OnEarlyPreInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyPreInitialize();
            }
        }

        private void OnPreInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.PreInitialize();
            }
        }

        private void OnLatePreInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LatePreInitialize();
            }
        }

        private void OnEarlyInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyInitialize();
            }
        }

        private void OnInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.Initialize();
            }
        }

        private void OnLateInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LateInitialize();
            }
        }

        private void OnEarlyPostInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyPostInitalize();
            }
        }

        private void OnPostInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.PostInitialize();
            }
        }

        private void OnLatePostInitialize()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LatePostInitialize();
            }
        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyPreTerminate();
            }
        }

        private void OnPreTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.PreTerminate();
            }
        }

        private void OnLatePreTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LatePreTerminate();
            }
        }

        private void OnEarlyTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyTerminate();
            }
        }

        private void OnTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.Terminate();
            }
        }

        private void OnLateTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LateTerminate();
            }
        }

        private void OnEarlyPostTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyPostTerminate();
            }
        }

        private void OnPostTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.PostTerminate();
            }
        }

        private void OnLatePostTerminate()
        {
            foreach (IModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LatePostTerminate();
            }
        }
        #endregion

        #endregion

        #region Methods
        /// <summary>
        /// Returns the module managers in the order they should be initialized.
        /// </summary>
        protected virtual IModuleManager[] GetModuleManagers()
        {
            return new IModuleManager[0];
        }
        #endregion

        #region Overrides
        protected override void PreConstruct()
        {
            base.PreConstruct();

            ModuleManagers = GetModuleManagers();
            ParentMainManager = (MainManager)ParentManager;

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

        protected override IManager GetParentManager()
        {
            return MainManager.Instance;
        }
        #endregion
    }
}