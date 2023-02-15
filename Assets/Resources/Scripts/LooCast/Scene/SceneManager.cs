using System;
using System.Collections;
using UnityEngine;

namespace LooCast.Scene
{
    using Game;
    using System.Collections.Generic;
    using UI.Screen;
    
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

        #region Static Methods
        public override void InitializeInstance()
        {
            base.InitializeInstance();
            UnityEngine.SceneManagement.SceneManager.sceneLoaded += OnSceneLoaded;
        }
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

        #region Coroutines
        private IEnumerator LoadSceneAsynchronously(string sceneIndex)
        {
            LoadingScreen loadingScreen = FindObjectOfType<LoadingScreen>();
            yield return loadingScreen.LoadSceneAsynchronously(sceneIndex);
        }
        #endregion
    }
}