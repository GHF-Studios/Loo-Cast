using UnityEngine;
using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public sealed class MainManager : Manager
    {
        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new MainManager();
                }
                return instance;
            }
        }
        #endregion
        
        #region Static Fields
        private static MainManager instance;
        #endregion

        #region Fields
        private List<ICoreModuleManager> coreModuleManagerChildrenList;
        #endregion

        #region Constructors
        private MainManager() : base("MainManager", null)
        {
            RegisterPreSetupAction(() =>
            {
                coreModuleManagerChildrenList = new List<ICoreModuleManager>();
            });

            RegisterSetupAction(() =>
            {
                // TODO:    Read the mod hierarchyFolder for valid core module managers and load them.
                //          This process is internal to the MainManager and thus there are no Methods to manage the child managers.

                coreModuleManagerChildrenList.Add(global::LooCast.System.SystemManager.Instance);
                coreModuleManagerChildrenList.Add(global::LooCast.Core.LooCastCoreManager.Instance);
            });

            RegisterPostSetupAction(() =>
            {
                RegisterEarlyPreInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnEarlyPreInitialize();
                    }
                });
                RegisterPreInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnPreInitialize();
                    }
                });
                RegisterLatePreInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnLatePreInitialize();
                    }
                });
                RegisterEarlyInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnEarlyInitialize();
                    }
                });
                RegisterInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnInitialize();
                    }
                });
                RegisterLateInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnLateInitialize();
                    }
                });
                RegisterEarlyPostInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnEarlyPostInitialize();
                    }
                });
                RegisterPostInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnPostInitialize();
                    }
                });
                RegisterLatePostInitializationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnLatePostInitialize();
                    }
                });

                RegisterEarlyPreTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnEarlyPreTerminate();
                    }
                });
                RegisterPreTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnPreTerminate();
                    }
                });
                RegisterLatePreTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnLatePreTerminate();
                    }
                });
                RegisterEarlyTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnEarlyTerminate();
                    }
                });
                RegisterTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnTerminate();
                    }
                });
                RegisterLateTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnLateTerminate();
                    }
                });
                RegisterEarlyPostTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnEarlyPostTerminate();
                    }
                });
                RegisterPostTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnPostTerminate();
                    }
                });
                RegisterLatePostTerminationAction(() =>
                {
                    foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                    {
                        coreModuleManager.OnLatePostTerminate();
                    }
                });

                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.OnPreSetup();
                    coreModuleManager.OnSetup();
                    coreModuleManager.OnPostSetup();
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
        #endregion
    }
}
