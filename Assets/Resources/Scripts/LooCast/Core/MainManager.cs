using System;
using System.Collections;
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
        #endregion

        #region Properties
        public int MaxFilamentsPerFrame => maxFilamentsPerFrame;
        public int MaxSectorsPerFrame => maxSectorsPerFrame;
        public int MaxRegionsPerFrame => maxRegionsPerFrame;
        public int MaxFilamentChunksPerFrame => maxFilamentChunksPerFrame;
        public int MaxSectorChunksPerFrame => maxSectorChunksPerFrame;
        public int MaxRegionChunksPerFrame => maxRegionChunksPerFrame;
        
        public int FilamentsPerFrame => filamentsPerFrame;
        public int SectorsPerFrame => sectorsPerFrame;
        public int RegionsPerFrame => regionsPerFrame;
        public int FilamentChunksPerFrame => filamentChunksPerFrame;
        public int SectorChunksPerFrame => sectorChunksPerFrame;
        public int RegionChunksPerFrame => regionChunksPerFrame;
        #endregion

        #region Fields
        [SerializeField] private int maxFilamentsPerFrame = 1;
        [SerializeField] private int maxSectorsPerFrame = 1;
        [SerializeField] private int maxRegionsPerFrame = 1;
        [SerializeField] private int maxFilamentChunksPerFrame = 1;
        [SerializeField] private int maxSectorChunksPerFrame = 1;
        [SerializeField] private int maxRegionChunksPerFrame = 1;

        private int filamentsPerFrame;
        private int sectorsPerFrame;
        private int regionsPerFrame;
        private int filamentChunksPerFrame;
        private int sectorChunksPerFrame;
        private int regionChunksPerFrame;
        private float frameTime;
        private int frameCount;
        private float frameRate;
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

            games = Games.Load();

            Debug.Log($"[MainManager] Initialized.");
            #endregion

            #region SteamManager Initialization
            _ = SteamManager.Initialized;
            #endregion

            #region TimerUtil Initialization
            TimerUtil.InitializeInstance();
            #endregion

            #region Utilities Initialization
            Universe.DensityMapGenerationUtil.InitializeInstance();
            #endregion

            #region TEMPORARY: Video Settings Initialization
            // TODO: Actually implement (video) settings
            if (Application.targetFrameRate == -1)
            {
                Debug.LogWarning("TargetFrameRate set to 60 by default, this has to be controlled by a not yet implemented video settings system!!!");
                Application.targetFrameRate = 60;
            }

            filamentsPerFrame = 1;
            sectorsPerFrame = 1;
            regionsPerFrame = 1;
            filamentChunksPerFrame = 1;
            sectorChunksPerFrame = 1;
            regionChunksPerFrame = 1;
            frameTime = 0.0f;
            frameCount = 0;
            frameRate = 0.0f;
            #endregion

            #region Scene Initialization
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

            Debug.Log($"[MainManager] Finished Initialization in Scene '{activeSceneName}'.");
        }

        private void Update()
        {
            frameCount++;
            frameTime += Time.deltaTime;

            if (frameTime > 1.0f)
            {
                frameRate = frameCount / frameTime;

                filamentsPerFrame = Mathf.RoundToInt((frameRate / Application.targetFrameRate).Map(0, 1, 0, maxFilamentsPerFrame));
                sectorsPerFrame = Mathf.RoundToInt((frameRate / Application.targetFrameRate).Map(0, 1, 0, maxSectorsPerFrame));
                regionsPerFrame = Mathf.RoundToInt((frameRate / Application.targetFrameRate).Map(0, 1, 0, maxRegionsPerFrame));
                filamentChunksPerFrame = Mathf.RoundToInt((frameRate / Application.targetFrameRate).Map(0, 1, 0, maxFilamentChunksPerFrame));
                sectorChunksPerFrame = Mathf.RoundToInt((frameRate / Application.targetFrameRate).Map(0, 1, 0, maxSectorChunksPerFrame));
                regionChunksPerFrame = Mathf.RoundToInt((frameRate / Application.targetFrameRate).Map(0, 1, 0, maxRegionChunksPerFrame));

                frameTime = 0.0f;
                frameCount = 0;
            }
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
                    GameManager.AddPostInitializationAction(postLoadAction);
                    Instance.StartCoroutine(Instance.LoadSceneAsynchronously(sceneName));
                    break;
                case SceneType.Game:
                    GameManager.AddPostInitializationAction(postLoadAction);
                    Instance.StartCoroutine(Instance.LoadSceneAsynchronously(sceneName));
                    break;
                default:
                    throw new ArgumentException($"Scene Type '{sceneName}' not supported!");
            }
            Debug.Log($"[MainManager] Finished loading Scene '{sceneName}'.");
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
        #endregion

        #region Methods
        public IEnumerator LoadSceneAsynchronously(string sceneIndex)
        {
            LoadingScreen loadingScreen = FindObjectOfType<LoadingScreen>();
            yield return loadingScreen.LoadSceneAsynchronously(sceneIndex);
        }
        #endregion
    }
}
