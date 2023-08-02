using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.ECS;

    public abstract class Manager : Entity, IManager
    {
        #region Classes
        new public class Data : Entity.Data, IManager.IData
        {
            #region Properties
            public string ManagerName { get; set; }
            public IManager ManagerParent { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public ManagerUnityComponent ManagerUnityComponent { get; private set; }
        
        public string ManagerName { get; private set; }

        IManager IChild<IManager>.Parent => ManagerParent;
        public IManager ManagerParent { get; private set; }
        
        IEnumerable<IManager> IParent<IManager>.Children => ManagerChildren;
        public IEnumerable<IManager> ManagerChildren => managerChildrenList;

        public bool IsEarlyPreInitializing { get; protected set; }
        public bool IsEarlyPreInitialized { get; protected set; }
        public bool IsLatePreInitializing { get; protected set; }
        public bool IsLatePreInitialized { get; protected set; }

        public bool IsEarlyInitializing { get; protected set; }
        public bool IsEarlyInitialized { get; protected set; }
        public bool IsLateInitializing { get; protected set; }
        public bool IsLateInitialized { get; protected set; }

        public bool IsEarlyPostInitializing { get; protected set; }
        public bool IsEarlyPostInitialized { get; protected set; }
        public bool IsLatePostInitializing { get; protected set; }
        public bool IsLatePostInitialized { get; protected set; }

        public bool IsEarlyPreTerminating { get; protected set; }
        public bool IsEarlyPreTerminated { get; protected set; }
        public bool IsLatePreTerminating { get; protected set; }
        public bool IsLatePreTerminated { get; protected set; }

        public bool IsEarlyTerminating { get; protected set; }
        public bool IsEarlyTerminated { get; protected set; }
        public bool IsLateTerminating { get; protected set; }
        public bool IsLateTerminated { get; protected set; }

        public bool IsEarlyPostTerminating { get; protected set; }
        public bool IsEarlyPostTerminated { get; protected set; }
        public bool IsLatePostTerminating { get; protected set; }
        public bool IsLatePostTerminated { get; protected set; }

        public bool IsPreSetupRunning { get; private set; }
        public bool IsPreSetupFinished { get; private set; }
        public bool IsSetupRunning { get; private set; }
        public bool IsSetupFinished { get; private set; }
        public bool IsPostSetupRunning { get; private set; }
        public bool IsPostSetupFinished { get; private set; }
        #endregion

        #region Fields
        private List<Action> earlyPreInitializationActions;
        private List<Action> latePreInitializationActions;

        private List<Action> earlyInitializationActions;
        private List<Action> lateInitializationActions;
        
        private List<Action> earlyPostInitializationActions;
        private List<Action> latePostInitializationActions;
        
        private List<Action> earlyPreTerminationActions;
        private List<Action> latePreTerminationActions;
        
        private List<Action> earlyTerminationActions;
        private List<Action> lateTerminationActions;
        
        private List<Action> earlyPostTerminationActions;
        private List<Action> latePostTerminationActions;

        private List<Action> preSetupActions;
        private List<Action> setupActions;
        private List<Action> postSetupActions;

        protected bool enableLogging = false;

        private List<IManager> managerChildrenList;
        #endregion

        #region Constructors
        /// <summary>
        /// Manager constructors are required be parameterless and should NEVER be called manually!
        /// </summary>
        protected Manager() : base()
        {
            preSetupActions = new List<Action>();
            setupActions = new List<Action>();
            postSetupActions = new List<Action>();
            
            earlyPreInitializationActions = new List<Action>();
            latePreInitializationActions = new List<Action>();
            earlyInitializationActions = new List<Action>();
            lateInitializationActions = new List<Action>();
            earlyPostInitializationActions = new List<Action>();
            latePostInitializationActions = new List<Action>();

            earlyPreTerminationActions = new List<Action>();
            latePreTerminationActions = new List<Action>();
            earlyTerminationActions = new List<Action>();
            lateTerminationActions = new List<Action>();
            earlyPostTerminationActions = new List<Action>();
            latePostTerminationActions = new List<Action>();
            
            RegisterPostSetupAction(() =>
            {
                EnableUnityBridge();
                UnityBridge.RootGameObject.name = ManagerName;
                ManagerUnityComponent = UnityBridge.RootGameObject.AddComponent<ManagerUnityComponent>();
                ManagerUnityComponent.Setup(this);

                if (ManagerParent != null)
                {
                    UnityBridge.RootGameObject.transform.SetParent(ManagerParent.UnityBridge.RootGameObject.transform);
                }
            });
        }
        #endregion

        #region Callbacks

        #region Setup Phases
        /// <summary>
        /// Automatically called after constructor invocation. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnPreSetup()
        {
            if (IsPreSetupRunning)
            {
                throw new InvalidOperationException("Cannot start pre-setup of the manager while the manager's pre-setup is already running!");
            }
            if (IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start pre-setup of the manager while the manager's pre-setup is already finished!");
            }
            if (IsSetupRunning)
            {
                throw new InvalidOperationException("Cannot start pre-setup of the manager while the manager's setup is already running!");
            }
            if (IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start pre-setup of the manager while the manager's setup is already finished!");
            }
            if (IsPostSetupRunning)
            {
                throw new InvalidOperationException("Cannot start pre-setup of the manager while the manager's post-setup is already running!");
            }
            if (IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start pre-setup of the manager while the manager's post-setup is already finished!");
            }

            IsPreSetupRunning = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting pre-setup.");
            }

            foreach (Action preSetupAction in preSetupActions)
            {
                preSetupAction.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished pre-setup.");
            }
            IsPreSetupRunning = false;
            IsPreSetupFinished = true;
        }

        /// <summary>
        /// Automatically called after OnPreSetup. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnSetup()
        {
            if (IsSetupRunning)
            {
                throw new InvalidOperationException("Cannot start setup of the manager while the manager's setup is already running!");
            }
            if (IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start setup of the manager while the manager's setup is already finished!");
            }
            if (IsPostSetupRunning)
            {
                throw new InvalidOperationException("Cannot start setup of the manager while the manager's post-setup is already running!");
            }
            if (IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start setup of the manager while the manager's post-setup is already finished!");
            }

            IsSetupRunning = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting setup.");
            }

            foreach (Action setupAction in setupActions)
            {
                setupAction.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished setup.");
            }
            IsSetupRunning = false;
            IsSetupFinished = true;
        }

        /// <summary>
        /// Automatically called after OnSetup. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnPostSetup()
        {
            if (IsPostSetupRunning)
            {
                throw new InvalidOperationException("Cannot start post-setup of the manager while the manager's post-setup is already running!");
            }
            if (IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start post-setup of the manager while the manager's post-setup is already finished!");
            }

            IsPostSetupRunning = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting post-setup.");
            }

            foreach (Action postSetupAction in postSetupActions)
            {
                postSetupAction.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished post-setup.");
            }
            IsPostSetupRunning = false;
            IsPostSetupFinished = true;
        }
        #endregion

        #region Initialization Phases
        /// <summary>
        /// Automatically called by parent manager.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnEarlyPreInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's post-setup is not finished!");
            }
            
            if (IsEarlyPreInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's early pre-initialization is already running!");
            }
            if (IsEarlyPreInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's early pre-initialization is already finished!");
            }
            if (IsPreInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's pre-initialization is already running!");
            }
            if (IsPreInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's pre-initialization is already finished!");
            }
            if (IsLatePreInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's late pre-initialization is already running!");
            }
            if (IsLatePreInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's late pre-initialization is already finished!");
            }
            if (IsEarlyInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's early initialization is already running!");
            }
            if (IsEarlyInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's early initialization is already finished!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's initialization is already running!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's initialization is already finished!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's late initialization is already running!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's late initialization is already finished!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's early post-initialization is already running!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's early post-initialization is already finished!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's post-initialization is already running!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's post-initialization is already finished!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start early pre-initialization of the manager while the manager's late post-initialization is already finished!");
            }

            IsEarlyPreInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Early Pre-Initialization.");
            }

            foreach (Action action in earlyPreInitializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Early Pre-Initialization.");
            }
            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
        }

        /// <summary>
        /// Automatically called after OnEarlyPreInitialize.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void OnPreInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's post-setup is not finished!");
            }

            if (IsPreInitializing)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's pre-initialization is already running!");
            }
            if (IsPreInitialized)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's pre-initialization is already finished!");
            }
            if (IsLatePreInitializing)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's late pre-initialization is already running!");
            }
            if (IsLatePreInitialized)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's late pre-initialization is already finished!");
            }
            if (IsEarlyInitializing)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's early initialization is already running!");
            }
            if (IsEarlyInitialized)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's early initialization is already finished!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's initialization is already running!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's initialization is already finished!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's late initialization is already running!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's late initialization is already finished!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's early post-initialization is already running!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's early post-initialization is already finished!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's post-initialization is already running!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's post-initialization is already finished!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start pre-initialization of the manager while the manager's late post-initialization is already finished!");
            }

            IsPreInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Pre-Initialization.");
            }

            foreach (Action action in preInitializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Pre-Initialization.");
            }
            IsPreInitializing = false;
            IsPreInitialized = true;
        }

        /// <summary>
        /// Automatically called after OnPreInitialize.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnLatePreInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's post-setup is not finished!");
            }

            if (IsLatePreInitializing)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's late pre-initialization is already running!");
            }
            if (IsLatePreInitialized)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's late pre-initialization is already finished!");
            }
            if (IsEarlyInitializing)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's early initialization is already running!");
            }
            if (IsEarlyInitialized)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's early initialization is already finished!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's initialization is already running!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's initialization is already finished!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's late initialization is already running!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's late initialization is already finished!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's early post-initialization is already running!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's early post-initialization is already finished!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's post-initialization is already running!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's post-initialization is already finished!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start late pre-initialization of the manager while the manager's late post-initialization is already finished!");
            }
            
            IsLatePreInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Late Pre-Initialization.");
            }

            foreach (Action action in latePreInitializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Late Pre-Initialization.");
            }
            IsLatePreInitializing = false;
            IsLatePreInitialized = true;
        }

        /// <summary>
        /// Automatically called after OnLatePreInitialize.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnEarlyInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's post-setup is not finished!");
            }

            if (IsEarlyInitializing)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's early initialization is already running!");
            }
            if (IsEarlyInitialized)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's early initialization is already finished!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's initialization is already running!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's initialization is already finished!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's late initialization is already running!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's late initialization is already finished!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's early post-initialization is already running!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's early post-initialization is already finished!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's post-initialization is already running!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's post-initialization is already finished!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start early initialization of the manager while the manager's late post-initialization is already finished!");
            }

            IsEarlyInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Early Initialization.");
            }

            foreach (Action action in earlyInitializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Early Initialization.");
            }
            IsEarlyInitializing = false;
            IsEarlyInitialized = true;
        }

        /// <summary>
        /// Automatically called after OnEarlyInitialize.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void OnInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's post-setup is not finished!");
            }

            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's initialization is already running!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's initialization is already finished!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's late initialization is already running!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's late initialization is already finished!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's early post-initialization is already running!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's early post-initialization is already finished!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's post-initialization is already running!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's post-initialization is already finished!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start initialization of the manager while the manager's late post-initialization is already finished!");
            }

            IsInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Initialization.");
            }

            foreach (Action action in initializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Initialization.");
            }
            IsInitializing = false;
            IsInitialized = true;
        }

        /// <summary>
        /// Automatically called after OnInitialize.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnLateInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's post-setup is not finished!");
            }

            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's late initialization is already running!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's late initialization is already finished!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's early post-initialization is already running!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's early post-initialization is already finished!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's post-initialization is already running!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's post-initialization is already finished!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start late initialization of the manager while the manager's late post-initialization is already finished!");
            }

            IsLateInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Late Initialization.");
            }

            foreach (Action action in lateInitializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Late Initialization.");
            }
            IsLateInitializing = false;
            IsLateInitialized = true;
        }

        /// <summary>
        /// Automatically called after OnLateInitialize.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnEarlyPostInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's post-setup is not finished!");
            }

            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's early post-initialization is already running!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's early post-initialization is already finished!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's post-initialization is already running!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's post-initialization is already finished!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start early post-initialization of the manager while the manager's late post-initialization is already finished!");
            }

            IsEarlyPostInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Early Post-Initialization.");
            }

            foreach (Action action in earlyPostInitializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Early Post-Initialization.");
            }
            IsEarlyPostInitializing = false;
            IsEarlyPostInitialized = true;
        }

        /// <summary>
        /// Automatically called after OnEarlyPostInitialize.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void OnPostInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start post-initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start post-initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start post-initialization of the manager while the manager's post-setup is not finished!");
            }

            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot start post-initialization of the manager while the manager's post-initialization is already running!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot start post-initialization of the manager while the manager's post-initialization is already finished!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start post-initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start post-initialization of the manager while the manager's late post-initialization is already finished!");
            }

            IsPostInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Post-Initialization.");
            }

            foreach (Action action in postInitializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Post-Initialization.");
            }
            IsPostInitializing = false;
            IsPostInitialized = true;
        }

        /// <summary>
        /// Automatically called after OnPostInitialize.
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnLatePostInitialize()
        {
            if (IsPreSetupRunning || !IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late post-initialization of the manager while the manager's pre-setup is not finished!");
            }
            if (IsSetupRunning || !IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late post-initialization of the manager while the manager's setup is not finished!");
            }
            if (IsPostSetupRunning || !IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot start late post-initialization of the manager while the manager's post-setup is not finished!");
            }

            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot start late post-initialization of the manager while the manager's late post-initialization is already running!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot start late post-initialization of the manager while the manager's late post-initialization is already finished!");
            }

            IsLatePostInitializing = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Late Post-Initialization.");
            }

            foreach (Action action in latePostInitializationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Late Post-Initialization.");
            }
            IsLatePostInitializing = false;
            IsLatePostInitialized = true;
        }
        #endregion

        #region Termination Phases
        /// <summary>
        /// Automatically called by parent manager via OnEarlyPreTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnEarlyPreTerminate()
        {
            if (IsEarlyPreTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's early pre-termination is already running!");
            }
            if (IsEarlyPreTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's early pre-termination is already finished!");
            }
            if (IsPreTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's pre-termination is already running!");
            }
            if (IsPreTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's pre-termination is already finished!");
            }
            if (IsLatePreTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's late pre-termination is already running!");
            }
            if (IsLatePreTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's late pre-termination is already finished!");
            }
            if (IsEarlyTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's early termination is already running!");
            }
            if (IsEarlyTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's early termination is already finished!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's termination is already running!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's termination is already finished!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's late termination is already running!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's late termination is already finished!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's early post-termination is already running!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's early post-termination is already finished!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's post-termination is already running!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's post-termination is already finished!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start early pre-termination of the manager while the manager's late post-termination is already finished!");
            }

            IsEarlyPreTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Early Pre-Termination.");
            }

            foreach (Action action in earlyPreTerminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Early Pre-Termination.");
            }
            IsEarlyPreTerminating = false;
            IsEarlyPreTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnEarlyPreTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void OnPreTerminate()
        {
            if (IsPreTerminating)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's pre-termination is already running!");
            }
            if (IsPreTerminated)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's pre-termination is already finished!");
            }
            if (IsLatePreTerminating)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's late pre-termination is already running!");
            }
            if (IsLatePreTerminated)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's late pre-termination is already finished!");
            }
            if (IsEarlyTerminating)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's early termination is already running!");
            }
            if (IsEarlyTerminated)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's early termination is already finished!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's termination is already running!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's termination is already finished!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's late termination is already running!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's late termination is already finished!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's early post-termination is already running!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's early post-termination is already finished!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's post-termination is already running!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's post-termination is already finished!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start pre-termination of the manager while the manager's late post-termination is already finished!");
            }

            IsPreTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Pre-Termination.");
            }

            foreach (Action action in preTerminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Pre-Termination.");
            }
            IsPreTerminating = false;
            IsPreTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnPreTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnLatePreTerminate()
        {
            if (IsLatePreTerminating)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's late pre-termination is already running!");
            }
            if (IsLatePreTerminated)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's late pre-termination is already finished!");
            }
            if (IsEarlyTerminating)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's early termination is already running!");
            }
            if (IsEarlyTerminated)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's early termination is already finished!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's termination is already running!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's termination is already finished!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's late termination is already running!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's late termination is already finished!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's early post-termination is already running!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's early post-termination is already finished!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's post-termination is already running!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's post-termination is already finished!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start late pre-termination of the manager while the manager's late post-termination is already finished!");
            }

            IsLatePreTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Late Pre-Termination.");
            }

            foreach (Action action in latePreTerminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Late Pre-Termination.");
            }
            IsLatePreTerminating = false;
            IsLatePreTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnLatePreTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnEarlyTerminate()
        {
            if (IsEarlyTerminating)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's early termination is already running!");
            }
            if (IsEarlyTerminated)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's early termination is already finished!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's termination is already running!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's termination is already finished!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's late termination is already running!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's late termination is already finished!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's early post-termination is already running!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's early post-termination is already finished!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's post-termination is already running!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's post-termination is already finished!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start early termination of the manager while the manager's late post-termination is already finished!");
            }
            
            IsEarlyTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Early Termination.");
            }

            foreach (Action action in earlyTerminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Early Termination.");
            }
            IsEarlyTerminating = false;
            IsEarlyTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnEarlyTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void OnTerminate()
        {
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's termination is already running!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's termination is already finished!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's late termination is already running!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's late termination is already finished!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's early post-termination is already running!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's early post-termination is already finished!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's post-termination is already running!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's post-termination is already finished!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start termination of the manager while the manager's late post-termination is already finished!");
            }
            
            IsTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Termination.");
            }

            foreach (Action action in terminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Termination.");
            }
            IsTerminating = false;
            IsTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnLateTerminate()
        {
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot start late termination of the manager while the manager's late termination is already running!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot start late termination of the manager while the manager's late termination is already finished!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot start late termination of the manager while the manager's early post-termination is already running!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot start late termination of the manager while the manager's early post-termination is already finished!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot start late termination of the manager while the manager's post-termination is already running!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot start late termination of the manager while the manager's post-termination is already finished!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start late termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start late termination of the manager while the manager's late post-termination is already finished!");
            }
            
            IsLateTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Late Termination.");
            }

            foreach (Action action in lateTerminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Late Termination.");
            }
            IsLateTerminating = false;
            IsLateTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnLateTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnEarlyPostTerminate()
        {
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot start early post-termination of the manager while the manager's early post-termination is already running!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot start early post-termination of the manager while the manager's early post-termination is already finished!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot start early post-termination of the manager while the manager's post-termination is already running!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot start early post-termination of the manager while the manager's post-termination is already finished!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start early post-termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start early post-termination of the manager while the manager's late post-termination is already finished!");
            }
            
            IsEarlyPostTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Early Post-Termination.");
            }

            foreach (Action action in earlyPostTerminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Early Post-Termination.");
            }
            IsEarlyPostTerminating = false;
            IsEarlyPostTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnEarlyPostTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void OnPostTerminate()
        {
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot start post-termination of the manager while the manager's post-termination is already running!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot start post-termination of the manager while the manager's post-termination is already finished!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start post-termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start post-termination of the manager while the manager's late post-termination is already finished!");
            }
            
            IsPostTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Post-Termination.");
            }

            foreach (Action action in postTerminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Post-Termination.");
            }
            IsPostTerminating = false;
            IsPostTerminated = true;
        }

        /// <summary>
        /// Automatically called after OnPostTerminate. 
        /// Do NOT manually call this method!
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void OnLatePostTerminate()
        {
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot start late post-termination of the manager while the manager's late post-termination is already running!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot start late post-termination of the manager while the manager's late post-termination is already finished!");
            }
            
            IsLatePostTerminating = true;
            string managerTypeName = GetType().Name;
            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Starting Late Post-Termination.");
            }

            foreach (Action action in latePostTerminationActions)
            {
                action.Invoke();
            }

            if (enableLogging)
            {
                Debug.Log($"[{managerTypeName}] Finished Late Post-Termination.");
            }
            IsLatePostTerminating = false;
            IsLatePostTerminated = true;
        }
        #endregion

        /// <summary>
        /// Automatically called when this manager is being created by the parent manager. 
        /// Do NOT manually call this method! 
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void OnCreate()
        {
            base.OnCreate();
        }

        /// <summary>
        /// Do NOT call this method, unless you are the LooCastApplication!
        /// Do NOT override this method, unless you are the MainManager!
        /// </summary>
        public override void OnDestroy()
        {
            throw new InvalidOperationException("Manager should not be destroyed manually! Only the parent manager should destroy it via the termination cycle!");
        }
        
        #endregion

        #region Methods

        #region Initialization Action Registration
        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterEarlyPreInitializationAction(Action action)
        {
            if (IsEarlyPreInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already early pre-initializing!");
            }
            if (IsEarlyPreInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already early pre-initialized!");
            }
            if (IsPreInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already pre-initializing!");
            }
            if (IsPreInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already pre-initialized!");
            }
            if (IsLatePreInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already late pre-initializing!");
            }
            if (IsLatePreInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already late pre-initialized!");
            }
            if (IsEarlyInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already early initializing!");
            }
            if (IsEarlyInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already early initialized!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already initialized!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already late initializing!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already late initialized!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already early post-initializing!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already early post-initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already post-initialized!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early pre-initialization action while the manager is already late post-initialized!");
            }

            earlyPreInitializationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void RegisterPreInitializationAction(Action preInitializationAction)
        {
            if (IsPreInitializing)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already pre-initializing!");
            }
            if (IsPreInitialized)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already pre-initialized!");
            }
            if (IsLatePreInitializing)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already late pre-initializing!");
            }
            if (IsLatePreInitialized)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already late pre-initialized!");
            }
            if (IsEarlyInitializing)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already early initializing!");
            }
            if (IsEarlyInitialized)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already early initialized!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already initialized!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already late initializing!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already late initialized!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already early post-initializing!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already early post-initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already post-initialized!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an pre-initialization action while the manager is already late post-initialized!");
            }

            preInitializationActions.Add(preInitializationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterLatePreInitializationAction(Action action)
        {
            if (IsLatePreInitializing)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already late pre-initializing!");
            }
            if (IsLatePreInitialized)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already late pre-initialized!");
            }
            if (IsEarlyInitializing)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already early initializing!");
            }
            if (IsEarlyInitialized)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already early initialized!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already initialized!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already late initializing!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already late initialized!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already early post-initializing!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already early post-initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already post-initialized!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an late pre-initialization action while the manager is already late post-initialized!");
            }
            
            latePreInitializationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterEarlyInitializationAction(Action action)
        {
            if (IsEarlyInitializing)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already early initializing!");
            }
            if (IsEarlyInitialized)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already early initialized!");
            }
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already initialized!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already late initializing!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already late initialized!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already early post-initializing!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already early post-initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already post-initialized!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early initialization action while the manager is already late post-initialized!");
            }
            
            earlyInitializationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void RegisterInitializationAction(Action initializationAction)
        {
            if (IsInitializing)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already initializing!");
            }
            if (IsInitialized)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already initialized!");
            }
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already late initializing!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already late initialized!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already early post-initializing!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already early post-initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already post-initialized!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an initialization action while the manager is already late post-initialized!");
            }

            initializationActions.Add(initializationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterLateInitializationAction(Action action)
        {
            if (IsLateInitializing)
            {
                throw new InvalidOperationException("Cannot register an late initialization action while the manager is already late initializing!");
            }
            if (IsLateInitialized)
            {
                throw new InvalidOperationException("Cannot register an late initialization action while the manager is already late initialized!");
            }
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an late initialization action while the manager is already early post-initializing!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an late initialization action while the manager is already early post-initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an late initialization action while the manager is already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an late initialization action while the manager is already post-initialized!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an late initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an late initialization action while the manager is already late post-initialized!");
            }
            
            lateInitializationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterEarlyPostInitializationAction(Action action)
        {
            if (IsEarlyPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early post-initialization action while the manager is already early post-initializing!");
            }
            if (IsEarlyPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early post-initialization action while the manager is already early post-initialized!");
            }
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early post-initialization action while the manager is already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early post-initialization action while the manager is already post-initialized!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an early post-initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an early post-initialization action while the manager is already late post-initialized!");
            }
            
            earlyPostInitializationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void RegisterPostInitializationAction(Action postInitializationAction)
        {
            if (IsPostInitializing)
            {
                throw new InvalidOperationException("Cannot register an post-initialization action while the manager is already post-initializing!");
            }
            if (IsPostInitialized)
            {
                throw new InvalidOperationException("Cannot register an post-initialization action while the manager is already post-initialized!");
            }
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an post-initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an post-initialization action while the manager is already late post-initialized!");
            }

            postInitializationActions.Add(postInitializationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterLatePostInitializationAction(Action action)
        {
            if (IsLatePostInitializing)
            {
                throw new InvalidOperationException("Cannot register an late post-initialization action while the manager is already late post-initializing!");
            }
            if (IsLatePostInitialized)
            {
                throw new InvalidOperationException("Cannot register an late post-initialization action while the manager is already late post-initialized!");
            }
            
            latePostInitializationActions.Add(action);
        }
        #endregion

        #region Termination Action Registration
        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterEarlyPreTerminationAction(Action action)
        {
            if (IsEarlyPreTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already early pre-terminating!");
            }
            if (IsEarlyPreTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already early pre-terminated!");
            }
            if (IsPreTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already pre-terminating!");
            }
            if (IsPreTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already pre-terminated!");
            }
            if (IsLatePreTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already late pre-terminating!");
            }
            if (IsLatePreTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already late pre-terminated!");
            }
            if (IsEarlyTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already early terminating!");
            }
            if (IsEarlyTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already early terminated!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already terminated!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already late terminating!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already late terminated!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already early post-terminating!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already early post-terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already post-terminated!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early pre-termination action while the manager is already late post-terminated!");
            }

            earlyPreTerminationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void RegisterPreTerminationAction(Action preTerminationAction)
        {
            if (IsPreTerminating)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already pre-terminating!");
            }
            if (IsPreTerminated)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already pre-terminated!");
            }
            if (IsLatePreTerminating)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already late pre-terminating!");
            }
            if (IsLatePreTerminated)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already late pre-terminated!");
            }
            if (IsEarlyTerminating)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already early terminating!");
            }
            if (IsEarlyTerminated)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already early terminated!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already terminated!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already late terminating!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already late terminated!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already early post-terminating!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already early post-terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already post-terminated!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an pre-termination action while the manager is already late post-terminated!");
            }

            preTerminationActions.Add(preTerminationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterLatePreTerminationAction(Action action)
        {
            if (IsLatePreTerminating)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already late pre-terminating!");
            }
            if (IsLatePreTerminated)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already late pre-terminated!");
            }
            if (IsEarlyTerminating)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already early terminating!");
            }
            if (IsEarlyTerminated)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already early terminated!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already terminated!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already late terminating!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already late terminated!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already early post-terminating!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already early post-terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already post-terminated!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an late pre-termination action while the manager is already late post-terminated!");
            }

            latePreTerminationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterEarlyTerminationAction(Action action)
        {
            if (IsEarlyTerminating)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already early terminating!");
            }
            if (IsEarlyTerminated)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already early terminated!");
            }
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already terminated!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already late terminating!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already late terminated!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already early post-terminating!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already early post-terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already post-terminated!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early termination action while the manager is already late post-terminated!");
            }

            earlyTerminationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void RegisterTerminationAction(Action terminationAction)
        {
            if (IsTerminating)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already terminating!");
            }
            if (IsTerminated)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already terminated!");
            }
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already late terminating!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already late terminated!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already early post-terminating!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already early post-terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already post-terminated!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an termination action while the manager is already late post-terminated!");
            }

            terminationActions.Add(terminationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterLateTerminationAction(Action action)
        {
            if (IsLateTerminating)
            {
                throw new InvalidOperationException("Cannot register an late termination action while the manager is already late terminating!");
            }
            if (IsLateTerminated)
            {
                throw new InvalidOperationException("Cannot register an late termination action while the manager is already late terminated!");
            }
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an late termination action while the manager is already early post-terminating!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an late termination action while the manager is already early post-terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an late termination action while the manager is already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an late termination action while the manager is already post-terminated!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an late termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an late termination action while the manager is already late post-terminated!");
            }

            lateTerminationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterEarlyPostTerminationAction(Action action)
        {
            if (IsEarlyPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early post-termination action while the manager is already early post-terminating!");
            }
            if (IsEarlyPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early post-termination action while the manager is already early post-terminated!");
            }
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early post-termination action while the manager is already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early post-termination action while the manager is already post-terminated!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an early post-termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an early post-termination action while the manager is already late post-terminated!");
            }

            earlyPostTerminationActions.Add(action);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public override void RegisterPostTerminationAction(Action postTerminationAction)
        {
            if (IsPostTerminating)
            {
                throw new InvalidOperationException("Cannot register an post-termination action while the manager is already post-terminating!");
            }
            if (IsPostTerminated)
            {
                throw new InvalidOperationException("Cannot register an post-termination action while the manager is already post-terminated!");
            }
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an post-termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an post-termination action while the manager is already late post-terminated!");
            }

            postTerminationActions.Add(postTerminationAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterLatePostTerminationAction(Action action)
        {
            if (IsLatePostTerminating)
            {
                throw new InvalidOperationException("Cannot register an late post-termination action while the manager is already late post-terminating!");
            }
            if (IsLatePostTerminated)
            {
                throw new InvalidOperationException("Cannot register an late post-termination action while the manager is already late post-terminated!");
            }

            latePostTerminationActions.Add(action);
        }
        #endregion

        #region Setup Action Registration
        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterPreSetupAction(Action preSetupAction)
        {
            if (IsPreSetupRunning)
            {
                throw new InvalidOperationException("Cannot register a pre-setup action while pre-setup is running!");
            }
            if (IsPreSetupFinished)
            {
                throw new InvalidOperationException("Cannot register a pre-setup action after pre-setup has finished!");
            }
            if (IsSetupRunning)
            {
                throw new InvalidOperationException("Cannot register a pre-setup action while setup is running!");
            }
            if (IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot register a pre-setup action after setup has finished!");
            }
            if (IsPostSetupRunning)
            {
                throw new InvalidOperationException("Cannot register a pre-setup action while post-setup is running!");
            }
            if (IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot register a pre-setup action after post-setup has finished!");
            }

            preSetupActions.Add(preSetupAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterSetupAction(Action setupAction)
        {
            if (IsSetupRunning)
            {
                throw new InvalidOperationException("Cannot register a setup action while setup is running!");
            }
            if (IsSetupFinished)
            {
                throw new InvalidOperationException("Cannot register a setup action after setup has finished!");
            }
            if (IsPostSetupRunning)
            {
                throw new InvalidOperationException("Cannot register a setup action while post-setup is running!");
            }
            if (IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot register a setup action after post-setup has finished!");
            }

            setupActions.Add(setupAction);
        }

        /// <summary>
        /// Only override this method if you know what you are doing!
        /// </summary>
        public virtual void RegisterPostSetupAction(Action postSetupAction)
        {
            if (IsPostSetupRunning)
            {
                throw new InvalidOperationException("Cannot register a post-setup action while post-setup is running!");
            }
            if (IsPostSetupFinished)
            {
                throw new InvalidOperationException("Cannot register a post-setup action after post-setup has finished!");
            }

            postSetupActions.Add(postSetupAction);
        }
        #endregion

        #region Data Management
        public override IEntity.IData GetEntityData()
        {
            IManager.IData managerData = (IManager.IData)base.GetEntityData();
            
            managerData.ManagerName = ManagerName;
            managerData.ManagerParent = ManagerParent;

            return managerData;
        }

        public override void SetEntityData(IEntity.IData data)
        {
            IManager.IData managerData = (IManager.IData)data;

            ManagerName = managerData.ManagerName;
            ManagerParent = managerData.ManagerParent;

            base.SetEntityData(data);
        }
        #endregion

        #endregion

        #region Overrides
        public override string ToString()
        {
            return $"Manager[{ManagerName ?? "Unnamed Manager"}]";
        }
        #endregion
    }
}