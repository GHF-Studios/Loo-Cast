using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    public abstract class Manager : Folder, IManager
    {
        #region Properties
        public string ManagerName => FolderName;
        public ManagerMonoBehaviour ManagerMonoBehaviour { get; private set; }

        IManager IChild<IManager>.Parent => (IManager)FolderParent;

        IEnumerable<IManager> IParent<IManager>.Children => (IEnumerable<IManager>)FolderChildren;
        #endregion

        #region Fields
        private List<Action> earlyPreInitializationActions;
        private List<Action> preInitializationActions;
        private List<Action> latePreInitializationActions;
        private List<Action> earlyInitializationActions;
        private List<Action> initializationActions;
        private List<Action> lateInitializationActions;
        private List<Action> earlyPostInitializationActions;
        private List<Action> postInitializationActions;
        private List<Action> latePostInitializationActions;
        
        private List<Action> earlyPreTerminationActions;
        private List<Action> preTerminationActions;
        private List<Action> latePreTerminationActions;
        private List<Action> earlyTerminationActions;
        private List<Action> terminationActions;
        private List<Action> lateTerminationActions;
        private List<Action> earlyPostTerminationActions;
        private List<Action> postTerminationActions;
        private List<Action> latePostTerminationActions;
        #endregion

        #region Constructors
        protected Manager(string managerName, IManager managerParent, ManagerMonoBehaviour managerMonoBehaviour) : base(managerName, managerParent)
        {
            if (managerParent != null)
            {
                managerMonoBehaviour.transform.SetParent(managerParent.ManagerMonoBehaviour.transform);
            }
            
            ManagerMonoBehaviour = managerMonoBehaviour;

            earlyPreInitializationActions = new List<Action>();
            preInitializationActions = new List<Action>();
            latePreInitializationActions = new List<Action>();
            earlyInitializationActions = new List<Action>();
            initializationActions = new List<Action>();
            lateInitializationActions = new List<Action>();
            earlyPostInitializationActions = new List<Action>();
            postInitializationActions = new List<Action>();
            latePostInitializationActions = new List<Action>();

            earlyPreTerminationActions = new List<Action>();
            preTerminationActions = new List<Action>();
            latePreTerminationActions = new List<Action>();
            earlyTerminationActions = new List<Action>();
            terminationActions = new List<Action>();
            lateTerminationActions = new List<Action>();
            earlyPostTerminationActions = new List<Action>();
            postTerminationActions = new List<Action>();
            latePostTerminationActions = new List<Action>();
        }
        #endregion

        #region Methods

        #region Initialization Phases
        public void EarlyPreInitialize()
        {
            IsEarlyPreInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Pre-Initialization.");

            foreach (Action action in earlyPreInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Pre-Initialization.");
            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
        }

        public void PreInitialize()
        {
            IsPreInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Pre-Initialization.");

            foreach (Action action in preInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Pre-Initialization.");
            IsPreInitializing = false;
            IsPreInitialized = true;
            LatePreInitialize();
        }

        public void LatePreInitialize()
        {
            IsLatePreInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Pre-Initialization.");

            foreach (Action action in latePreInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Pre-Initialization.");
            IsLatePreInitializing = false;
            IsLatePreInitialized = true;
        }

        public void EarlyInitialize()
        {
            IsEarlyInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Initialization.");

            foreach (Action action in earlyInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Initialization.");
            IsEarlyInitializing = false;
            IsEarlyInitialized = true;
        }

        public void Initialize()
        {
            IsInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Initialization.");

            foreach (Action action in initializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Initialization.");
            IsInitializing = false;
            IsInitialized = true;
            LateInitialize();
        }

        public void LateInitialize()
        {
            IsLateInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Initialization.");

            foreach (Action action in lateInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Initialization.");
            IsLateInitializing = false;
            IsLateInitialized = true;
        }

        public void EarlyPostInitalize()
        {
            IsEarlyPostInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Post-Initialization.");

            foreach (Action action in earlyPostInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Post-Initialization.");
            IsEarlyPostInitializing = false;
            IsEarlyPostInitialized = true;
        }

        public void PostInitialize()
        {
            IsPostInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Post-Initialization.");

            foreach (Action action in postInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Post-Initialization.");
            IsPostInitializing = false;
            IsPostInitialized = true;
            LatePostInitialize();
        }

        public void LatePostInitialize()
        {
            IsLatePostInitializing = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Post-Initialization.");

            foreach (Action action in latePostInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Post-Initialization.");
            IsLatePostInitializing = false;
            IsLatePostInitialized = true;
        }
        #endregion

        #region Termination Phases
        public void EarlyPreTerminate()
        {
            IsEarlyPreTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Pre-Termination.");

            foreach (Action action in earlyPreTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Pre-Termination.");
            IsEarlyPreTerminating = false;
            IsEarlyPreTerminated = true;
            PreTerminate();
        }

        public void PreTerminate()
        {
            IsPreTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Pre-Termination.");

            foreach (Action action in preTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Pre-Termination.");
            IsPreTerminating = false;
            IsPreTerminated = true;
            LatePreTerminate();
        }

        public void LatePreTerminate()
        {
            IsLatePreTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Pre-Termination.");

            foreach (Action action in latePreTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Pre-Termination.");
            IsLatePreTerminating = false;
            IsLatePreTerminated = true;
            EarlyTerminate();
        }

        public void EarlyTerminate()
        {
            IsEarlyTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Termination.");

            foreach (Action action in earlyTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Termination.");
            IsEarlyTerminating = false;
            IsEarlyTerminated = true;
            Terminate();
        }

        public void Terminate()
        {
            IsTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Termination.");

            foreach (Action action in terminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Termination.");
            IsTerminating = false;
            IsTerminated = true;
            LateTerminate();
        }

        public void LateTerminate()
        {
            IsLateTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Termination.");

            foreach (Action action in lateTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Termination.");
            IsLateTerminating = false;
            IsLateTerminated = true;
            EarlyPostTerminate();
        }

        public void EarlyPostTerminate()
        {
            IsEarlyPostTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Post-Termination.");

            foreach (Action action in earlyPostTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Post-Termination.");
            IsEarlyPostTerminating = false;
            IsEarlyPostTerminated = true;
            PostTerminate();
        }

        public void PostTerminate()
        {
            IsPostTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Post-Termination.");

            foreach (Action action in postTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Post-Termination.");
            IsPostTerminating = false;
            IsPostTerminated = true;
            LatePostTerminate();
        }

        public void LatePostTerminate()
        {
            IsLatePostTerminating = true;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Post-Termination.");

            foreach (Action action in latePostTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Post-Termination.");
            IsLatePostTerminating = false;
            IsLatePostTerminated = true;
        }
        #endregion

        #region Initialization Action Registration
        public void RegisterEarlyPreInitializationAction(Action action)
        {
            earlyPreInitializationActions.Add(action);
        }

        public void RegisterPreInitializationAction(Action action)
        {
            preInitializationActions.Add(action);
        }

        public void RegisterLatePreInitializationAction(Action action)
        {
            latePreInitializationActions.Add(action);
        }

        public void RegisterEarlyInitializationAction(Action action)
        {
            earlyInitializationActions.Add(action);
        }

        public void RegisterInitializationAction(Action action)
        {
            initializationActions.Add(action);
        }

        public void RegisterLateInitializationAction(Action action)
        {
            lateInitializationActions.Add(action);
        }

        public void RegisterEarlyPostInitializationAction(Action action)
        {
            earlyPostInitializationActions.Add(action);
        }

        public void RegisterPostInitializationAction(Action action)
        {
            postInitializationActions.Add(action);
        }

        public void RegisterLatePostInitializationAction(Action action)
        {
            latePostInitializationActions.Add(action);
        }
        #endregion

        #region Termination Action Registration
        public void RegisterEarlyPreTerminationAction(Action action)
        {
            earlyPreTerminationActions.Add(action);
        }

        public void RegisterPreTerminationAction(Action action)
        {
            preTerminationActions.Add(action);
        }

        public void RegisterLatePreTerminationAction(Action action)
        {
            latePreTerminationActions.Add(action);
        }

        public void RegisterEarlyTerminationAction(Action action)
        {
            earlyTerminationActions.Add(action);
        }

        public void RegisterTerminationAction(Action action)
        {
            terminationActions.Add(action);
        }

        public void RegisterLateTerminationAction(Action action)
        {
            lateTerminationActions.Add(action);
        }

        public void RegisterEarlyPostTerminationAction(Action action)
        {
            earlyPostTerminationActions.Add(action);
        }

        public void RegisterPostTerminationAction(Action action)
        {
            postTerminationActions.Add(action);
        }

        public void RegisterLatePostTerminationAction(Action action)
        {
            latePostTerminationActions.Add(action);
        }
        #endregion
        
        #endregion
    }
}