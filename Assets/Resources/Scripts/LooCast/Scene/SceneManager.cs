using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Scene
{
    using LooCast.System;
    using LooCast.System.Management;
    using LooCast.Game;
    using LooCast.UI.Screen;

    public class SceneManager : ModuleManager
    {
        #region Enums
        public enum SceneType
        {
            MainMenu,
            Game
        }
        #endregion

        #region Static Properties
        public static SceneManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[SceneManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<SceneManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static SceneManager instance;
        #endregion

        #region Fields
        private Queue<Action> postSceneLoadActionQueue;
        #endregion

        #region Callbacks
        private void OnSceneLoaded(UnityEngine.SceneManagement.Scene scene, UnityEngine.SceneManagement.LoadSceneMode mode)
        {
            ExecutePostSceneLoadActions();
            Debug.Log($"[SceneManager] Finished loading Scene '{scene.name}'.");
        }
        #endregion

        #region Methods
        public void LoadScene(SceneType sceneType, Action postSceneLoadAction = null)
        {
            string sceneName = Enum.GetName(typeof(SceneType), sceneType);
            Debug.Log($"[SceneManager] Loading Scene '{sceneName}'.");
            switch (sceneType)
            {
                case SceneType.MainMenu:
                    if (UnityEngine.SceneManagement.SceneManager.GetActiveScene().name == "Game")
                    {
                        GameManager.SaveGame();
                    }

                    AddPostSceneLoadAction(postSceneLoadAction);
                    Instance.StartCoroutine(Instance.LoadSceneAsynchronously(sceneName));
                    break;
                case SceneType.Game:
                    AddPostSceneLoadAction(postSceneLoadAction);
                    Instance.StartCoroutine(Instance.LoadSceneAsynchronously(sceneName));
                    break;
                default:
                    throw new ArgumentException($"[SceneManager] Scene Type '{sceneName}' not supported!");
            }
        }

        public void AddPostSceneLoadAction(Action postSceneLoadAction)
        {
            if (postSceneLoadAction == null)
            {
                return;
            }

            if (postSceneLoadActionQueue == null)
            {
                postSceneLoadActionQueue = new Queue<Action>();
            }

            postSceneLoadActionQueue.Enqueue(postSceneLoadAction);
        }

        private void ExecutePostSceneLoadActions()
        {
            if (postSceneLoadActionQueue == null)
            {
                return;
            }

            while (postSceneLoadActionQueue.Count > 0)
            {
                postSceneLoadActionQueue.Dequeue().Invoke();
            }
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Scene", rootNamespace);
            looCastType = new Type(typeof(SceneManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);
            #endregion

            UnityEngine.SceneManagement.SceneManager.sceneLoaded += OnSceneLoaded;
        }
        #endregion

        #region Coroutines
        private IEnumerator LoadSceneAsynchronously(string sceneIndex)
        {
            LoadingScreen loadingScreen = FindObjectOfType<LoadingScreen>();
            yield return loadingScreen.LoadSceneAsynchronously(sceneIndex);
        }
        #endregion
    }
}