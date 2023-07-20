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
            coreModuleManagerChildrenList = new List<ICoreModuleManager>();

            RegisterEarlyPreInitializationAction(() =>
            {
                coreModuleManagerChildrenList.Add(global::LooCast.System.SystemManager.Instance);
                coreModuleManagerChildrenList.Add(global::LooCast.Core.LooCastCoreManager.Instance);

                // TODO:    Read the mod hierarchyFolder for valid core module managers and load them.
                //          This process is internal to the MainManager and thus there are no Methods to manage the child managers.

                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.EarlyPreInitialize();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.PreInitialize();
                }
            });

            RegisterLatePreInitializationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.LatePreInitialize();
                }
            });

            RegisterEarlyInitializationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.EarlyInitialize();
                }
            });

            RegisterInitializationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.Initialize();
                }
            });

            RegisterLateInitializationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.LateInitialize();
                }
            });

            RegisterEarlyPostInitializationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.EarlyPostInitialize();
                }
            });

            RegisterPostInitializationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.PostInitialize();
                }
            });

            RegisterLatePostInitializationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.LatePostInitialize();
                }
            });

            RegisterEarlyPreTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.EarlyPreTerminate();
                }
            });

            RegisterPreTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.PreTerminate();
                }
            });

            RegisterLatePreTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.LatePreTerminate();
                }
            });

            RegisterEarlyTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.EarlyTerminate();
                }
            });

            RegisterTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.Terminate();
                }
            });

            RegisterLateTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.LateTerminate();
                }
            });

            RegisterEarlyPostTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.EarlyPostTerminate();
                }
            });

            RegisterPostTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.PostTerminate();
                }
            });

            RegisterLatePostTerminationAction(() =>
            {
                foreach (ICoreModuleManager coreModuleManager in coreModuleManagerChildrenList)
                {
                    coreModuleManager.LatePostTerminate();
                }
            });
        }
        #endregion

        #region Methods
        public void OnPreAwake()
        {
            EarlyPreInitialize();
            PreInitialize();
            LatePreInitialize();
        }

        public void OnAwake()
        {
            EarlyInitialize();
            Initialize();
            LateInitialize();
        }

        public void OnPostAwake()
        {
            EarlyPostInitialize();
            PostInitialize();
            LatePostInitialize();
        }

        public void OnDisable()
        {
            if (!IsEarlyPreTerminating && !IsPreTerminating && !IsLatePreTerminating && !IsEarlyTerminating && !IsTerminating && !IsLateTerminating && !IsEarlyPostTerminating && !IsPostTerminating && !IsLatePostTerminating)
            {
                EarlyPreTerminate();
                PreTerminate();
                LatePreTerminate();
                EarlyTerminate();
                Terminate();
                LateTerminate();
                EarlyPostTerminate();
                PostTerminate();
                LatePostTerminate();
            }
        }

        public void OnApplicationQuit()
        {
            if (!IsEarlyPreTerminating && !IsPreTerminating && !IsLatePreTerminating && !IsEarlyTerminating && !IsTerminating && !IsLateTerminating && !IsEarlyPostTerminating && !IsPostTerminating && !IsLatePostTerminating)
            {
                EarlyPreTerminate();
                PreTerminate();
                LatePreTerminate();
                EarlyTerminate();
                Terminate();
                LateTerminate();
                EarlyPostTerminate();
                PostTerminate();
                LatePostTerminate();
            }
        }
        #endregion
    }
}
