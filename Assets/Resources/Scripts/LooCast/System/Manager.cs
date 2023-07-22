using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.Serialization;
    using LooCast.System.ECS;
    using LooCast.System.Paths;

    public class Manager : Entity
    {
        #region Classes
        new public class Data : Entity.Data
        {
            #region Properties
            public string ManagerName { get; set; }
            #endregion

            #region Constructors
            public Data(string assemblyQualifiedEntityTypeName) : base(assemblyQualifiedEntityTypeName)
            {
                ManagerName = "Root";
            }

            public Data(string assemblyQualifiedEntityTypeName, IComponent.IData[] componentDatas, string managerName) : base(assemblyQualifiedEntityTypeName, componentDatas)
            {
                ManagerName = managerName;
            }
            #endregion
        }
        #endregion

        #region Properties
        public ManagerUnityComponent ManagerUnityComponent { get; private set; }
        
        public string ManagerName { get; private set; }

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

        protected bool enableLogging = false;
        
        #endregion

        #region Constructors
        /// <summary>
        /// Manager constructors are required be parameterless and should NEVER be called manually!
        /// </summary>
        public Manager()
        {
            // preSetupActions = new List<Action>();
            // setupActions = new List<Action>();
            // postSetupActions = new List<Action>();
            // 
            // earlyPreInitializationActions = new List<Action>();
            // latePreInitializationActions = new List<Action>();
            // earlyInitializationActions = new List<Action>();
            // lateInitializationActions = new List<Action>();
            // earlyPostInitializationActions = new List<Action>();
            // latePostInitializationActions = new List<Action>();
            // 
            // earlyPreTerminationActions = new List<Action>();
            // latePreTerminationActions = new List<Action>();
            // earlyTerminationActions = new List<Action>();
            // lateTerminationActions = new List<Action>();
            // earlyPostTerminationActions = new List<Action>();
            // latePostTerminationActions = new List<Action>();
            
            // RegisterSetupAction(() =>
            // {
            //     EnableUnityBridge();
            //     UnityBridge.RootGameObject.name = ManagerName;
            //     ManagerUnityComponent = UnityBridge.RootGameObject.AddComponent<ManagerUnityComponent>();
            //     ManagerUnityComponent.Setup(this);
            // 
            //     if (ManagerParent != null)
            //     {
            //         UnityBridge.RootGameObject.transform.SetParent(ManagerParent.UnityBridge.RootGameObject.transform);
            //     }
            // });
        }
        #endregion

        #region Callbacks
        
        #endregion

        #region Methods
        

        #endregion

        #region Overrides
        public override string ToString()
        {
            return ManagerName;
        }
        #endregion
    }
}