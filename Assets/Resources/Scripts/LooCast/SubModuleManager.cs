using System;
using UnityEngine;
using LooCast;

namespace LooCast
{
    public abstract class SubModuleManager : ModuleManager
    {
        #region Constructors
        protected SubModuleManager(SubModuleManager[] dependencies, SubModuleManager[] subModuleManagers) : base(dependencies, subModuleManagers)
        {
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
            
        }

        private void OnPreInitialize()
        {

        }

        private void OnLatePreInitialize()
        {

        }

        private void OnEarlyInitialize()
        {

        }

        private void OnInitialize()
        {

        }

        private void OnLateInitialize()
        {

        }

        private void OnEarlyPostInitialize()
        {

        }

        private void OnPostInitialize()
        {

        }

        private void OnLatePostInitialize()
        {

        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {

        }

        private void OnPreTerminate()
        {

        }

        private void OnLatePreTerminate()
        {

        }

        private void OnEarlyTerminate()
        {

        }

        private void OnTerminate()
        {

        }

        private void OnLateTerminate()
        {

        }

        private void OnEarlyPostTerminate()
        {

        }

        private void OnPostTerminate()
        {

        }

        private void OnLatePostTerminate()
        {

        }
        #endregion

        #endregion
    }
}