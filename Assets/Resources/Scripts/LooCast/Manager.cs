using System.Reflection;
using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast
{
    using Core;
    
    public abstract class Manager : MonoBehaviour, INamespaceProvider, ITypeProvider, ISingletonInstanceProvider
    {
        #region Static Properties

        #region Initialization Phase Flags
        public bool IsEarlyPreInitializing { get; private set; }
        public bool IsPreInitializing { get; private set; }
        public bool IsLatePreInitializing { get; private set; }
        public bool IsEarlyPreInitialized { get; private set; }
        public bool IsPreInitialized { get; private set; }
        public bool IsLatePreInitialized { get; private set; }

        public bool IsEarlyInitializing { get; private set; }
        public bool IsInitializing { get; private set; }
        public bool IsLateInitializing { get; private set; }
        public bool IsEarlyInitialized { get; private set; }
        public bool IsInitialized { get; private set; }
        public bool IsLateInitialized { get; private set; }

        public bool IsEarlyPostInitializing { get; private set; }
        public bool IsPostInitializing { get; private set; }
        public bool IsLatePostInitializing { get; private set; }
        public bool IsEarlyPostInitialized { get; private set; }
        public bool IsPostInitialized { get; private set; }
        public bool IsLatePostInitialized { get; private set; }

        public bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public bool IsCompletelyInitialized
        {
            get
            {
                return IsFullyPreInitialized && IsFullyInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Termination Phase Flags
        public bool IsEarlyPreTerminating { get; private set; }
        public bool IsPreTerminating { get; private set; }
        public bool IsLatePreTerminating { get; private set; }
        public bool IsEarlyPreTerminated { get; private set; }
        public bool IsPreTerminated { get; private set; }
        public bool IsLatePreTerminated { get; private set; }

        public bool IsEarlyTerminating { get; private set; }
        public bool IsTerminating { get; private set; }
        public bool IsLateTerminating { get; private set; }
        public bool IsEarlyTerminated { get; private set; }
        public bool IsTerminated { get; private set; }
        public bool IsLateTerminated { get; private set; }

        public bool IsEarlyPostTerminating { get; private set; }
        public bool IsPostTerminating { get; private set; }
        public bool IsLatePostTerminating { get; private set; }
        public bool IsEarlyPostTerminated { get; private set; }
        public bool IsPostTerminated { get; private set; }
        public bool IsLatePostTerminated { get; private set; }

        public bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }
        #endregion

        #endregion

        #region Properties
        public Namespace LooCastNamespace => looCastNamespace;
        public Type LooCastType => looCastType;
        public Instance LooCastInstance => looCastInstance;

        public Manager[] Dependencies { get; private set; }
        #endregion

        #region Fields
        protected Namespace looCastNamespace;
        protected Type looCastType;
        protected Instance looCastInstance;

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

        #region Methods

        #region Initialization Phases
        public void EarlyPreInitialize()
        {
            IsEarlyPreInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsEarlyPreInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Early Pre-Initialized in time!");
                }
            }

            foreach (Action action in earlyPreInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");
            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
        }

        public void PreInitialize()
        {
            IsPreInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsPreInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Pre-Initialized in time!");
                }
            }

            foreach (Action action in preInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Pre-Initialization in Scene '{activeSceneName}'.");
            IsPreInitializing = false;
            IsPreInitialized = true;
        }

        public void LatePreInitialize()
        {
            IsLatePreInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Pre-Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsLatePreInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Late Pre-Initialized in time!");
                }
            }

            foreach (Action action in latePreInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Pre-Initialization in Scene '{activeSceneName}'.");
            IsLatePreInitializing = false;
            IsLatePreInitialized = true;
        }

        public void EarlyInitialize()
        {
            IsEarlyInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsEarlyInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Early Initialized in time!");
                }
            }

            foreach (Action action in earlyInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Initialization in Scene '{activeSceneName}'.");
            IsEarlyInitializing = false;
            IsEarlyInitialized = true;
        }

        public void Initialize()
        {
            IsInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Initialized in time!");
                }
            }

            foreach (Action action in initializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Initialization in Scene '{activeSceneName}'.");
            IsInitializing = false;
            IsInitialized = true;
        }

        public void LateInitialize()
        {
            IsLateInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsLateInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Late Initialized in time!");
                }
            }

            foreach (Action action in lateInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Initialization in Scene '{activeSceneName}'.");
            IsLateInitializing = false;
            IsLateInitialized = true;
        }

        public void EarlyPostInitalize()
        {
            IsEarlyPostInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Post-Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsEarlyPostInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Early Post-Initialized in time!");
                }
            }

            foreach (Action action in earlyPostInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Post-Initialization in Scene '{activeSceneName}'.");
            IsEarlyPostInitializing = false;
            IsEarlyPostInitialized = true;
        }

        public void PostInitialize()
        {
            IsPostInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Post-Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsPostInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Post-Initialized in time!");
                }
            }

            foreach (Action action in postInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Post-Initialization in Scene '{activeSceneName}'.");
            IsPostInitializing = false;
            IsPostInitialized = true;
        }

        public void LatePostInitialize()
        {
            IsLatePostInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Post-Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsLatePostInitialized)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Late Post-Initialized in time!");
                }
            }

            foreach (Action action in latePostInitializationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Post-Initialization in Scene '{activeSceneName}'.");
            IsLatePostInitializing = false;
            IsLatePostInitialized = true;
        }
        #endregion

        #region Termination Phases
        public void EarlyPreTerminate()
        {
            IsEarlyPreTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Pre-Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsEarlyPreTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Early Pre-Terminated in time!");
                }
            }

            foreach (Action action in earlyPreTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Pre-Termination in Scene '{activeSceneName}'.");
            IsEarlyPreTerminating = false;
            IsEarlyPreTerminated = true;
        }

        public void PreTerminate()
        {
            IsPreTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Pre-Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsPreTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Pre-Terminated in time!");
                }
            }

            foreach (Action action in preTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Pre-Termination in Scene '{activeSceneName}'.");
            IsPreTerminating = false;
            IsPreTerminated = true;
        }

        public void LatePreTerminate()
        {
            IsLatePreTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Pre-Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsLatePreTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Late Pre-Terminated in time!");
                }
            }

            foreach (Action action in latePreTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Pre-Termination in Scene '{activeSceneName}'.");
            IsLatePreTerminating = false;
            IsLatePreTerminated = true;
        }

        public void EarlyTerminate()
        {
            IsEarlyTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsEarlyTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Early Terminated in time!");
                }
            }

            foreach (Action action in earlyTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Termination in Scene '{activeSceneName}'.");
            IsEarlyTerminating = false;
            IsEarlyTerminated = true;
        }

        public void Terminate()
        {
            IsTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Terminated in time!");
                }
            }

            foreach (Action action in terminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Termination in Scene '{activeSceneName}'.");
            IsTerminating = false;
            IsTerminated = true;
        }

        public void LateTerminate()
        {
            IsLateTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsLateTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Late Terminated in time!");
                }
            }

            foreach (Action action in lateTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Termination in Scene '{activeSceneName}'.");
            IsLateTerminating = false;
            IsLateTerminated = true;
        }

        public void EarlyPostTerminate()
        {
            IsEarlyPostTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Post-Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsEarlyPostTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Early Post-Terminated in time!");
                }
            }

            foreach (Action action in earlyPostTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Early Post-Termination in Scene '{activeSceneName}'.");
            IsEarlyPostTerminating = false;
            IsEarlyPostTerminated = true;
        }

        public void PostTerminate()
        {
            IsPostTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Post-Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsPostTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Post-Terminated in time!");
                }
            }

            foreach (Action action in postTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Post-Termination in Scene '{activeSceneName}'.");
            IsPostTerminating = false;
            IsPostTerminated = true;
        }

        public void LatePostTerminate()
        {
            IsLatePostTerminating = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Late Post-Termination in Scene '{activeSceneName}'.");

            foreach (Manager dependency in Dependencies)
            {
                if (!dependency.IsLatePostTerminated)
                {
                    throw new ExecutionOrderException($"[{managerTypeName}] Dependency '{dependency.GetType().Name}' in Scene '{activeSceneName}' has not been Late Post-Terminated in time!");
                }
            }

            foreach (Action action in latePostTerminationActions)
            {
                action.Invoke();
            }

            Debug.Log($"[{managerTypeName}] Finished Late Post-Termination in Scene '{activeSceneName}'.");
            IsLatePostTerminating = false;
            IsLatePostTerminated = true;
        }
        #endregion

        #region Initialization Action Registration
        protected void RegisterEarlyPreInitializationAction(Action action)
        {
            earlyPreInitializationActions.Add(action);
        }

        protected void RegisterPreInitializationAction(Action action)
        {
            preInitializationActions.Add(action);
        }

        protected void RegisterLatePreInitializationAction(Action action)
        {
            latePreInitializationActions.Add(action);
        }

        protected void RegisterEarlyInitializationAction(Action action)
        {
            earlyInitializationActions.Add(action);
        }

        protected void RegisterInitializationAction(Action action)
        {
            initializationActions.Add(action);
        }

        protected void RegisterLateInitializationAction(Action action)
        {
            lateInitializationActions.Add(action);
        }

        protected void RegisterEarlyPostInitializationAction(Action action)
        {
            earlyPostInitializationActions.Add(action);
        }

        protected void RegisterPostInitializationAction(Action action)
        {
            postInitializationActions.Add(action);
        }

        protected void RegisterLatePostInitializationAction(Action action)
        {
            latePostInitializationActions.Add(action);
        }
        #endregion

        #region Termination Action Registration
        protected void RegisterEarlyPreTerminationAction(Action action)
        {
            earlyPreTerminationActions.Add(action);
        }

        protected void RegisterPreTerminationAction(Action action)
        {
            preTerminationActions.Add(action);
        }

        protected void RegisterLatePreTerminationAction(Action action)
        {
            latePreTerminationActions.Add(action);
        }

        protected void RegisterEarlyTerminationAction(Action action)
        {
            earlyTerminationActions.Add(action);
        }

        protected void RegisterTerminationAction(Action action)
        {
            terminationActions.Add(action);
        }

        protected void RegisterLateTerminationAction(Action action)
        {
            lateTerminationActions.Add(action);
        }

        protected void RegisterEarlyPostTerminationAction(Action action)
        {
            earlyPostTerminationActions.Add(action);
        }

        protected void RegisterPostTerminationAction(Action action)
        {
            postTerminationActions.Add(action);
        }

        protected void RegisterLatePostTerminationAction(Action action)
        {
            latePostTerminationActions.Add(action);
        }
        #endregion

        public virtual void PreInitializeInstance()
        {
            Dependencies = GetDependencies();

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

        public virtual void InitializeInstance()
        {
            
        }

        public virtual void PostInitializeInstance()
        {
            
        }

        /// <summary>
        /// Returns the dependencies in no particular order.
        /// </summary>
        protected virtual Manager[] GetDependencies()
        {
            return new Manager[0];
        }
        #endregion
    }
}