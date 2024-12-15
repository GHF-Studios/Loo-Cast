using System;
using UnityEngine;
using LooCast;
using LooCast.System.Identifiers;

namespace LooCast.System
{
    public abstract class CoreModuleManager : ModuleManager
    {
        #region Properties
        public ModuleManager[] ModuleManagers { get; private set; }
        #endregion

        #region Constructors
        protected CoreModuleManager(TypeIdentifier typeIdentifier, GameObject parentGameObject = null) : base(typeIdentifier, parentGameObject)
        {
            
        }
        #endregion

        #region Callbacks

        #region Initialization Phases
        private void OnEarlyPreInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyPreInitialize();
            }
        }

        private void OnPreInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.PreInitialize();
            }
        }

        private void OnLatePreInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LatePreInitialize();
            }
        }

        private void OnEarlyInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyInitialize();
            }
        }

        private void OnInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.Initialize();
            }
        }

        private void OnLateInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LateInitialize();
            }
        }

        private void OnEarlyPostInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyPostInitalize();
            }
        }

        private void OnPostInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.PostInitialize();
            }
        }

        private void OnLatePostInitialize()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LatePostInitialize();
            }
        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyPreTerminate();
            }
        }

        private void OnPreTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.PreTerminate();
            }
        }

        private void OnLatePreTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LatePreTerminate();
            }
        }

        private void OnEarlyTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyTerminate();
            }
        }

        private void OnTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.Terminate();
            }
        }

        private void OnLateTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LateTerminate();
            }
        }

        private void OnEarlyPostTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.EarlyPostTerminate();
            }
        }

        private void OnPostTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.PostTerminate();
            }
        }

        private void OnLatePostTerminate()
        {
            foreach (ModuleManager moduleManager in ModuleManagers)
            {
                moduleManager.LatePostTerminate();
            }
        }
        #endregion

        #endregion

        #region Methods
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            ModuleManagers = GetModuleManagers();

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
        /// Returns the module managers in the order they should be initialized.
        /// </summary>
        protected virtual ModuleManager[] GetModuleManagers()
        {
            return new ModuleManager[0];
        }
        #endregion
    }
}