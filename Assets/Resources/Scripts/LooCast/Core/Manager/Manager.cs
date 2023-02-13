using System.Reflection;
using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.Core.Manager
{
    using Core;
    
    public abstract class Manager : MonoBehaviour
    {
        #region Static Properties

        #region Initialization Phase Flags
        public static bool IsEarlyPreInitializing { get; private set; }
        public static bool IsPreInitializing { get; private set; }
        public static bool IsLatePreInitializing { get; private set; }
        public static bool IsEarlyPreInitialized { get; private set; }
        public static bool IsPreInitialized { get; private set; }
        public static bool IsLatePreInitialized { get; private set; }

        public static bool IsEarlyInitializing { get; private set; }
        public static bool IsInitializing { get; private set; }
        public static bool IsLateInitializing { get; private set; }
        public static bool IsEarlyInitialized { get; private set; }
        public static bool IsInitialized { get; private set; }
        public static bool IsLateInitialized { get; private set; }

        public static bool IsEarlyPostInitializing { get; private set; }
        public static bool IsPostInitializing { get; private set; }
        public static bool IsLatePostInitializing { get; private set; }
        public static bool IsEarlyPostInitialized { get; private set; }
        public static bool IsPostInitialized { get; private set; }
        public static bool IsLatePostInitialized { get; private set; }

        public static bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public static bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public static bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public static bool IsCompletelyInitialized
        {
            get
            {
                return IsFullyPreInitialized && IsFullyInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Termination Phase Flags
        public static bool IsEarlyPreTerminating { get; private set; }
        public static bool IsPreTerminating { get; private set; }
        public static bool IsLatePreTerminating { get; private set; }
        public static bool IsEarlyPreTerminated { get; private set; }
        public static bool IsPreTerminated { get; private set; }
        public static bool IsLatePreTerminated { get; private set; }

        public static bool IsEarlyTerminating { get; private set; }
        public static bool IsTerminating { get; private set; }
        public static bool IsLateTerminating { get; private set; }
        public static bool IsEarlyTerminated { get; private set; }
        public static bool IsTerminated { get; private set; }
        public static bool IsLateTerminated { get; private set; }

        public static bool IsEarlyPostTerminating { get; private set; }
        public static bool IsPostTerminating { get; private set; }
        public static bool IsLatePostTerminating { get; private set; }
        public static bool IsEarlyPostTerminated { get; private set; }
        public static bool IsPostTerminated { get; private set; }
        public static bool IsLatePostTerminated { get; private set; }

        public static bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public static bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public static bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public static bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }
        #endregion

        #endregion

        #region Constructors
        protected Manager(Manager[] dependencies)
        {
            this.dependencies = dependencies;
        }
        #endregion

        #region Fields
        private Manager[] dependencies;
        
        private List<Action> earlyPreInitializationActions = new List<Action>();
        private List<Action> preInitializationActions = new List<Action>();
        private List<Action> latePreInitializationActions = new List<Action>();
        private List<Action> earlyInitializationActions = new List<Action>();
        private List<Action> initializationActions = new List<Action>();
        private List<Action> lateInitializationActions = new List<Action>();
        private List<Action> earlyPostInitializationActions = new List<Action>();
        private List<Action> postInitializationActions = new List<Action>();
        private List<Action> latePostInitializationActions = new List<Action>();
        
        private List<Action> earlyPreTerminationActions = new List<Action>();
        private List<Action> preTerminationActions = new List<Action>();
        private List<Action> latePreTerminationActions = new List<Action>();
        private List<Action> earlyTerminationActions = new List<Action>();
        private List<Action> terminationActions = new List<Action>();
        private List<Action> lateTerminationActions = new List<Action>();
        private List<Action> earlyPostTerminationActions = new List<Action>();
        private List<Action> postTerminationActions = new List<Action>();
        private List<Action> latePostTerminationActions = new List<Action>();
        #endregion

        #region Methods

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

        #region Initialization Phases
        public void EarlyPreInitialize()
        {
            IsEarlyPreInitializing = true;
            string activeSceneName = SceneManager.GetActiveScene().name;
            string managerTypeName = GetType().Name;
            Debug.Log($"[{managerTypeName}] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyEarlyPreInitializedField = dependency.GetType().GetField("IsEarlyPreInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyEarlyPreInitialized = (bool)IsDependencyEarlyPreInitializedField.GetValue(null);
                if (!IsDependencyEarlyPreInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyPreInitializedField = dependency.GetType().GetField("IsPreInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyPreInitialized = (bool)IsDependencyPreInitializedField.GetValue(null);
                if (!IsDependencyPreInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyLatePreInitializedField = dependency.GetType().GetField("IsLatePreInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyLatePreInitialized = (bool)IsDependencyLatePreInitializedField.GetValue(null);
                if (!IsDependencyLatePreInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyEarlyInitializedField = dependency.GetType().GetField("IsEarlyInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyEarlyInitialized = (bool)IsDependencyEarlyInitializedField.GetValue(null);
                if (!IsDependencyEarlyInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyInitializedField = dependency.GetType().GetField("IsInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyInitialized = (bool)IsDependencyInitializedField.GetValue(null);
                if (!IsDependencyInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyLateInitializedField = dependency.GetType().GetField("IsLateInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyLateInitialized = (bool)IsDependencyLateInitializedField.GetValue(null);
                if (!IsDependencyLateInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyEarlyPostInitializedField = dependency.GetType().GetField("IsEarlyPostInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyEarlyPostInitialized = (bool)IsDependencyEarlyPostInitializedField.GetValue(null);
                if (!IsDependencyEarlyPostInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyPostInitializedField = dependency.GetType().GetField("IsPostInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyPostInitialized = (bool)IsDependencyPostInitializedField.GetValue(null);
                if (!IsDependencyPostInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyLatePostInitializedField = dependency.GetType().GetField("IsLatePostInitialized", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyLatePostInitialized = (bool)IsDependencyLatePostInitializedField.GetValue(null);
                if (!IsDependencyLatePostInitialized)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyEarlyPreTerminatedField = dependency.GetType().GetField("IsEarlyPreTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyEarlyPreTerminated = (bool)IsDependencyEarlyPreTerminatedField.GetValue(null);
                if (!IsDependencyEarlyPreTerminated)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyPreTerminatedField = dependency.GetType().GetField("IsPreTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyPreTerminated = (bool)IsDependencyPreTerminatedField.GetValue(null);
                if (!IsDependencyPreTerminated)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyLatePreTerminatedField = dependency.GetType().GetField("IsLatePreTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyLatePreTerminated = (bool)IsDependencyLatePreTerminatedField.GetValue(null);
                if (!IsDependencyLatePreTerminated)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyEarlyTerminatedField = dependency.GetType().GetField("IsEarlyTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyEarlyTerminated = (bool)IsDependencyEarlyTerminatedField.GetValue(null);
                if (!IsDependencyEarlyTerminated)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyTerminatedField = dependency.GetType().GetField("IsTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyTerminated = (bool)IsDependencyTerminatedField.GetValue(null);
                if (!IsDependencyTerminated)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyLateTerminatedField = dependency.GetType().GetField("IsLateTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyLateTerminated = (bool)IsDependencyLateTerminatedField.GetValue(null);
                if (!IsDependencyLateTerminated)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyEarlyPostTerminatedField = dependency.GetType().GetField("IsEarlyPostTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyEarlyPostTerminated = (bool)IsDependencyEarlyPostTerminatedField.GetValue(null);
                if (!IsDependencyEarlyPostTerminated)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyPostTerminatedField = dependency.GetType().GetField("IsPostTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyPostTerminated = (bool)IsDependencyPostTerminatedField.GetValue(null);
                if (!IsDependencyPostTerminated)
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

            foreach (Manager dependency in dependencies)
            {
                FieldInfo IsDependencyLatePostTerminatedField = dependency.GetType().GetField("IsLatePostTerminated", BindingFlags.Static | BindingFlags.Public);
                bool IsDependencyLatePostTerminated = (bool)IsDependencyLatePostTerminatedField.GetValue(null);
                if (!IsDependencyLatePostTerminated)
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

        #endregion
    }
}