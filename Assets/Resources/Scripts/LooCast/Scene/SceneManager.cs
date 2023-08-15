using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.Scene
{
    using LooCast.System;
    using LooCast.System.ECS;
    using LooCast.Core;
    using global::System.Collections;

    public sealed class SceneManager : ModuleManager
    {
        #region Delegates
        public delegate void SceneEvent(SceneType sceneType);
        public delegate void ProgressEvent(float progress);
        #endregion

        #region Events
        public event SceneEvent OnSceneLoadStart;
        public event SceneEvent OnSceneLoadFinish;
        public event ProgressEvent OnSceneLoadProgress;
        #endregion

        #region Static Properties
        public static SceneManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<SceneManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static SceneManager instance;
        #endregion

        #region Properties
        public bool IsLoadingScene { get; private set; } = false;
        #endregion

        #region Fields
        private Dictionary<SceneType, string> sceneNames;
        #endregion

        #region Constructors
        public SceneManager() : base()
        {
            sceneNames = new Dictionary<SceneType, string>
            {
                { SceneType.MainMenu, "MainMenu" },
                { SceneType.Game, "Game" },
                { SceneType.Editor, "Editor" }
            };

            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedSceneManagerEntityTypeName = typeof(SceneManager).AssemblyQualifiedName;
                string assemblyQualifiedSceneManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedSceneManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSceneManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSceneManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSceneManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSceneManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSceneManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSceneManagerEntityDataTypeName;
                instanceData.ManagerName = "SceneManager";
                instanceData.ManagerParent = LooCastCoreManager.Instance;

                SetEntityMetaData(instanceMetaData);
                SetEntityData(instanceData);

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component's data here

                // Register pre-included components here

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPostSetup();
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
        }
        #endregion

        #region Methods
        public void LoadSceneAsync(SceneType sceneType)
        {
            if (IsLoadingScene)
            {
                throw new InvalidOperationException("Cannot load scene while another scene is loading!");
            }

            IsLoadingScene = true;
            if (OnSceneLoadStart != null)
            {
                OnSceneLoadStart.Invoke(sceneType);
            }
            string sceneName = sceneNames[sceneType];
            AsyncOperation asyncOperation = UnityEngine.SceneManagement.SceneManager.LoadSceneAsync(sceneName);
            ManagerUnityComponent.RunCoroutine(TrackLoadingProgress(asyncOperation, sceneType));
        }
        
        private IEnumerator TrackLoadingProgress(AsyncOperation asyncOperation, SceneType sceneType)
        {
            while (!asyncOperation.isDone)
            {
                float progress = Mathf.Clamp01(asyncOperation.progress / 0.9f);
                Debug.Log("[SceneManager] Scene Loading Progress: " + progress * 100 + "%");
                if (OnSceneLoadProgress != null)
                {
                    OnSceneLoadProgress.Invoke(progress);
                }
                yield return null;
            }

            if (OnSceneLoadFinish != null)
            {
                OnSceneLoadFinish.Invoke(sceneType);
            }
            IsLoadingScene = false;
        }
        #endregion
    }
}
