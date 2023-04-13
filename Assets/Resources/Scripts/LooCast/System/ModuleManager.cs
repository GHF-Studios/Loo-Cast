using System;
using UnityEngine;

namespace LooCast.System
{
    using global::LooCast.System.MetaData;
    
    public abstract class ModuleManager<ModuleManagerType, ModuleManagerMetaDataType> : Manager<ModuleManagerType, ModuleManagerMetaDataType>, IModuleManager
        where ModuleManagerType : ModuleManager<ModuleManagerType, ModuleManagerMetaDataType>, new()
        where ModuleManagerMetaDataType : ModuleManagerMetaData, new()
    {
        #region Properties
        public ISubModuleManager[] SubModuleManagers { get; private set; }
        public ICoreModuleManager ParentCoreModuleManager { get; private set; }
        #endregion

        #region Callbacks

        #region Initialization Phases
        private void OnEarlyPreInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyPreInitialize();
            }
        }

        private void OnPreInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.PreInitialize();
            }
        }

        private void OnLatePreInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LatePreInitialize();
            }
        }

        private void OnEarlyInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyInitialize();
            }
        }

        private void OnInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.Initialize();
            }
        }

        private void OnLateInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LateInitialize();
            }
        }

        private void OnEarlyPostInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyPostInitalize();
            }
        }

        private void OnPostInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.PostInitialize();
            }
        }

        private void OnLatePostInitialize()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LatePostInitialize();
            }
        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyPreTerminate();
            }
        }

        private void OnPreTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.PreTerminate();
            }
        }

        private void OnLatePreTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LatePreTerminate();
            }
        }

        private void OnEarlyTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyTerminate();
            }
        }

        private void OnTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.Terminate();
            }
        }

        private void OnLateTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LateTerminate();
            }
        }

        private void OnEarlyPostTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.EarlyPostTerminate();
            }
        }

        private void OnPostTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.PostTerminate();
            }
        }

        private void OnLatePostTerminate()
        {
            foreach (ISubModuleManager subModuleManager in SubModuleManagers)
            {
                subModuleManager.LatePostTerminate();
            }
        }
        #endregion

        #endregion

        #region Methods
        /// <summary>
        /// Returns the sub-module managers in the order they should be initialized.
        /// </summary>
        protected virtual ISubModuleManager[] GetISubModuleManagers()
        {
            return new ISubModuleManager[0];
        }
        #endregion

        #region Overrides
        protected override void PreConstruct()
        {
            base.PreConstruct();

            SubModuleManagers = GetISubModuleManagers();
            ParentCoreModuleManager = (ICoreModuleManager)ParentManager;

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
    }
}