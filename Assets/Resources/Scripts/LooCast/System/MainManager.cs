using UnityEngine;
using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    
    public sealed class MainManager : Manager
    {
        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<MainManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion
        
        #region Static Fields
        private static MainManager instance;
        #endregion

        #region Fields
        private List<CoreModuleManager> coreModuleManagerChildrenList;
        #endregion

        #region Constructors
        public MainManager() : base()
        {
            coreModuleManagerChildrenList = new List<CoreModuleManager>();

            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedMainManagerEntityTypeName = typeof(MainManager).AssemblyQualifiedName;
                string assemblyQualifiedMainManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedMainManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData mainManagerMetaData = new Entity.MetaData();
                mainManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainManagerEntityTypeName;
                mainManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainManagerEntityMetaDataTypeName;
                mainManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainManagerEntityDataTypeName;
                mainManagerMetaData.EntityID = new Guid();

                Manager.Data mainManagerData = new Manager.Data();
                mainManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainManagerEntityTypeName;
                mainManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainManagerEntityMetaDataTypeName;
                mainManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainManagerEntityDataTypeName;
                mainManagerData.ManagerName = "MainManager";
                mainManagerData.ManagerParent = null;

                SetEntityMetaData(mainManagerMetaData);
                SetEntityData(mainManagerData);


                // TODO:    Read the mod hierarchyFolder for valid core module managers and load them.
                //          This process is internal to the MainManager and thus there are no Methods to manage the child managers.

                coreModuleManagerChildrenList.Add(global::LooCast.System.SystemManager.Instance);
                coreModuleManagerChildrenList.Add(global::LooCast.Core.LooCastCoreManager.Instance);

                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component's data here

                // Register pre-included components here

                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnPostSetup();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                // Pre-Initialize pre-included components here
            });

            RegisterInitializationAction(() =>
            {
                // Initialize pre-included components here
            });

            RegisterPostInitializationAction(() =>
            {
                // Post-Initialize pre-included components here
            });

            RegisterEarlyPreInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnEarlyPreInitialize();
                }
            });
            RegisterPreInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnPreInitialize();
                }
            });
            RegisterLatePreInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnLatePreInitialize();
                }
            });
            RegisterEarlyInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnEarlyInitialize();
                }
            });
            RegisterInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnInitialize();
                }
            });
            RegisterLateInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnLateInitialize();
                }
            });
            RegisterEarlyPostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnEarlyPostInitialize();
                }
            });
            RegisterPostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnPostInitialize();
                }
            });
            RegisterLatePostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnLatePostInitialize();
                }
            });

            RegisterEarlyPreTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnEarlyPreTerminate();
                }
            });
            RegisterPreTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnPreTerminate();
                }
            });
            RegisterLatePreTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnLatePreTerminate();
                }
            });
            RegisterEarlyTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnEarlyTerminate();
                }
            });
            RegisterTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnTerminate();
                }
            });
            RegisterLateTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnLateTerminate();
                }
            });
            RegisterEarlyPostTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnEarlyPostTerminate();
                }
            });
            RegisterPostTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnPostTerminate();
                }
            });
            RegisterLatePostTerminationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnLatePostTerminate();
                }
            });
        }
        #endregion

        #region Callbacks
        /// <summary>
        /// Automatically called when the MainManager is being created by the LooCastApplication. 
        /// Do NOT manually call this method! 
        /// </summary>
        public sealed override void OnCreate()
        {
            base.OnCreate();
        }

        /// <summary>
        /// Automatically called after OnCreate. 
        /// Do NOT manually call this method!
        /// </summary>
        public sealed override void OnPreSetup()
        {
            base.OnPreSetup();
        }

        /// <summary>
        /// Automatically called after OnPreSetup. 
        /// Do NOT manually call this method!
        /// </summary>
        public sealed override void OnSetup()
        {
            base.OnSetup();
        }

        /// <summary>
        /// Automatically called after OnSetup. 
        /// Do NOT manually call this method!
        /// </summary>
        public sealed override void OnPostSetup()
        {
            base.OnPostSetup();
        }

        /// <summary>
        /// Automatically called after OnPostSetup. 
        /// Do NOT manually call this method!
        /// </summary>
        public void OnPreAwake()
        {
            OnEarlyPreInitialize();
            OnPreInitialize();
            OnLatePreInitialize();
        }

        /// <summary>
        /// Automatically called after OnPreAwake. 
        /// Do NOT manually call this method!
        /// </summary>
        public void OnAwake()
        {
            OnEarlyInitialize();
            OnInitialize();
            OnLateInitialize();
        }

        /// <summary>
        /// Automatically called after OnAwake. 
        /// Do NOT manually call this method!
        /// </summary>
        public void OnPostAwake()
        {
            OnEarlyPostInitialize();
            OnPostInitialize();
            OnLatePostInitialize();
        }

        /// <summary>
        /// Automatically called when the MainManager is being destroyed by the LooCastApplication. 
        /// Do NOT manually call this method! 
        /// </summary>
        public sealed override void OnDestroy()
        {
            IsDestroyed = true;
            
            OnEarlyPreTerminate();
            OnPreTerminate();
            OnLatePreTerminate();
            OnEarlyTerminate();
            OnTerminate();
            OnLateTerminate();
            OnEarlyPostTerminate();
            OnPostTerminate();
            OnLatePostTerminate();
        }
        #endregion

        #region Overrides
        public override void EnableUnityBridge()
        {
            base.EnableUnityBridge();
            
            UnityEngine.Object.DontDestroyOnLoad(UnityBridge.RootGameObject);
        }
        #endregion
    }
}
