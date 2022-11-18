using System;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.Core
{
    using MainMenu;
    using Game;
    using Util;
    using System.Collections.Generic;
    using UnityEditor.Build.Content;

    public class MainManager : MonoBehaviour
    {
        #region Enums
        public enum SceneType
        {
            MainMenu,
            Game
        }
        #endregion

        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[MainManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<MainManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        public static Games Games => games;
        #endregion

        #region Static Fields
        private static MainManager instance;
        #endregion

        #region Properties
        private static Games games;
        private static Game selectedGame;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
            Debug.Log($"[MainManager] Starting Initialization in Scene '{activeSceneName}'.");

            #region Initialization

            #region MainManager Initialization
            if (instance != null)
            {
                Destroy(gameObject);
                return;
            }
            instance = this;
            DontDestroyOnLoad(this);

            games = Games.LoadGames();

            Debug.Log($"[MainManager] Initialized.");
            #endregion

            #region SteamManager Initialization
            _ = SteamManager.Initialized;
            #endregion

            #region TimerUtil Initialization
            TimerUtil.InitializeInstance();
            #endregion

            #region Scene Initialization
            InitializeScene(activeSceneName);
            #endregion

            #endregion

            Debug.Log($"[MainManager] Finished Initialization in Scene '{activeSceneName}'.");
        }
        #endregion

        #region Static Methods

        public static void LoadScene(SceneType sceneType, Action postLoadAction = null)
        {
            string sceneName = Enum.GetName(typeof(SceneType), sceneType);
            Debug.Log($"[MainManager] Loading Scene '{sceneName}'.");
            switch (sceneType)
            {
                case SceneType.MainMenu:
                    Instance.StartCoroutine(FindObjectOfType<UI.Screen.LoadingScreen>().LoadSceneAsynchronously(sceneName, () =>
                    {
                        InitializeScene("MainMenu");
                        postLoadAction?.Invoke();
                    }));
                    break;
                case SceneType.Game:
                    Instance.StartCoroutine(FindObjectOfType<UI.Screen.LoadingScreen>().LoadSceneAsynchronously(sceneName, () =>
                    {
                        InitializeScene("Game");
                        postLoadAction?.Invoke();
                    }));
                    break;
                default:
                    throw new ArgumentException($"Scene Type '{sceneName}' not supported!");
            }
            Debug.Log($"[MainManager] Finished loading Scene '{sceneName}'.");
        }

        public static void CreateNewGame(string gameName)
        {
            Game newGame = new Game(gameName);
            games.AddGame(newGame);
        }

        public static void LoadGame(string gameName)
        {
            LoadScene(SceneType.Game, () =>
            {
                GameManager.LoadGame(games.GetGame(gameName));
            });
        }

        public static void DeleteGame(string gameName)
        {
            Game game = games.GetGame(gameName);
            Game.DeleteGame(game);
        }

        public static void RenameGame(string oldGameName, string newGameName)
        {
            Game game = games.GetGame(oldGameName);
            Game.Rename(game, newGameName);
        }

        public static void LoadMainMenu()
        {
            LoadScene(SceneType.MainMenu);
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreInitialize()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
            Debug.Log($"[MainManager] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Pre-Initialization

            #endregion

            Debug.Log($"[MainManager] Finished Pre-Initialization in Scene '{activeSceneName}'.");
            _ = Instance;
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void PostInitialize()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
            Debug.Log($"[MainManager] Starting Post-Initialization in Scene '{activeSceneName}'.");

            #region Post-Initialization

            #endregion

            Debug.Log($"[MainManager] Finished Post-Initialization in Scene '{activeSceneName}'.");
        }

        private static void InitializeScene(string sceneName)
        {
            switch (sceneName)
            {
                case "MainMenu":
                    MainMenuManager mainMenuManager = FindObjectOfType<MainMenuManager>();
                    mainMenuManager.Initialize();
                    break;
                case "Game":
                    GameManager gameManager = FindObjectOfType<GameManager>();
                    gameManager.Initialize();
                    break;
                default:
                    throw new NotImplementedException($"Scene Initialization has not been implemented for Scene '{sceneName}'!");
            }
        }
        #endregion

        #region Methods

        #endregion
    }
}
