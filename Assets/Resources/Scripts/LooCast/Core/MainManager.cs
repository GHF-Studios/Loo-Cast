using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.Core
{
    using MainMenu;
    using Game;
    using Util;
    using Universe;
    using UI.Screen;
    using Math.Map;
    using LooCast.Data;

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
        public static Game GameToBeLoaded => gameToBeLoaded;    // TODO: Implement this
        #endregion

        #region Static Fields
        private static MainManager instance;
        private static Games games;
        private static Game gameToBeLoaded;
        public static float saveInterval = 30.0f;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            Debug.Log($"[MainManager] Starting Initialization.");

            #region Initialization

            #region Data.Path
            _ = Data.Path;
            #endregion
            
            #region MainManager
            if (instance != null)
            {
                Destroy(gameObject);
                return;
            }
            instance = this;
            DontDestroyOnLoad(this);

            games = Games.Load();

            Debug.Log($"[MainManager] Initialized.");
            #endregion

            #region SteamManager
            _ = SteamManager.Initialized;
            #endregion

            #region ModManager
            ModManager.Instance.Initialize();
            #endregion

            #region Utilities
            TimerUtil.InitializeInstance();
            Universe.DensityMapGenerationUtil.InitializeInstance();
            #endregion

            #region Scene
            string activeSceneName = SceneManager.GetActiveScene().name;
            switch (activeSceneName)
            {
                case "MainMenu":
                    break;
                case "Game":
                    GameManager.AddPostInitializationAction(() =>
                    {
                        GameManager gameManager = FindObjectOfType<GameManager>();
                        if (games.Contains("New Game"))
                        {
                            gameManager.InitializeGame(games.GetGame("New Game"));
                        }
                        else
                        {
                            gameManager.InitializeGame("New Game");
                        }
                    });
                    break;
            }
            #endregion

            #endregion

            Debug.Log($"[MainManager] Finished Initialization.");
        }
        #endregion

        #region Static Methods
        public static void CreateNewGame(string gameName)
        {
            if (games.Contains(gameName))
            {
                throw new Exception("Cannot create new Game, because another Game with the same Name already exists!");
            }

            LoadScene(SceneType.Game, () =>
            {
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(gameName);
            });
        }

        public static void CreateNewGame(string gameName, Universe.GenerationSettings generationSettings)
        {
            if (games.Contains(gameName))
            {
                throw new Exception("Cannot create new Game, because another Game with the same Name already exists!");
            }

            LoadScene(SceneType.Game, () =>
            {
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(gameName, generationSettings);
            });
        }

        public static void LoadGame(string gameName)
        {
            if (!games.Contains(gameName))
            {
                throw new Exception("Cannot load Game, because it does not exist!");
            }

            LoadScene(SceneType.Game, () =>
            {
                Game game = games.GetGame(gameName);
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(game);
            });
        }

        public static void DeleteGame(string gameName)
        {
            if (GameManager.Initialized && GameManager.Instance.CurrentGame.Name == gameName)
            {
                throw new Exception("Cannot delete Game when it is loaded!");
            }

            Game game = games.GetGame(gameName);
            Game.DeleteGame(game);
        }

        public static void RenameGame(string oldGameName, string newGameName)
        {
            if (GameManager.Initialized && GameManager.Instance.CurrentGame.Name == oldGameName)
            {
                throw new Exception("Cannot rename Game when it is loaded!");
            }

            Game game = games.GetGame(oldGameName);
            Game.Rename(game, newGameName);
        }

        public static void LoadMainMenu()
        {
            LoadScene(SceneType.MainMenu);
        }

        private static void LoadScene(SceneType sceneType, Action postLoadAction = null)
        {
            string sceneName = Enum.GetName(typeof(SceneType), sceneType);
            Debug.Log($"[MainManager] Loading Scene '{sceneName}'.");
            switch (sceneType)
            {
                case SceneType.MainMenu:
                    if (SceneManager.GetActiveScene().name == "Game")
                    {
                        Game.Save(GameManager.Instance.CurrentGame);
                    }

                    GameManager.AddPostInitializationAction(postLoadAction);
                    Instance.StartCoroutine(Instance.LoadSceneAsynchronously(sceneName));
                    break;
                case SceneType.Game:
                    GameManager.AddPostInitializationAction(postLoadAction);
                    Instance.StartCoroutine(Instance.LoadSceneAsynchronously(sceneName));
                    break;
                default:
                    throw new ArgumentException($"[MainManager] Scene Type '{sceneName}' not supported!");
            }
            Debug.Log($"[MainManager] Finished loading Scene '{sceneName}'.");
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreInitialize()
        {
            Debug.Log($"[MainManager] Starting Pre-Initialization.");

            #region Pre-Initialization
            
            #region ModManager
            _ = ModManager.Instance;
            ModManager.Instance.OnPreInitialize();
            #endregion

            #region ModuleManager
            _ = ModuleManager.Instance;
            #endregion
            
            #endregion

            Debug.Log($"[MainManager] Finished Pre-Initialization.");
            _ = Instance;
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void PostInitialize()
        {
            Debug.Log($"[MainManager] Starting Post-Initialization.");

            #region Post-Initialization

            #region ModManager
            ModManager.Instance.OnPostInitialize();
            #endregion

            #endregion

            Debug.Log($"[MainManager] Finished Post-Initialization'.");
        }
        #endregion

        #region Methods
        #endregion

        #region Coroutines
        public IEnumerator LoadSceneAsynchronously(string sceneIndex)
        {
            LoadingScreen loadingScreen = FindObjectOfType<LoadingScreen>();
            yield return loadingScreen.LoadSceneAsynchronously(sceneIndex);
        }
        #endregion
    }
}
