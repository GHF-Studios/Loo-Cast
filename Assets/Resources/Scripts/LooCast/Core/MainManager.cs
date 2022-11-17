using System;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.Core
{
    using MainMenu;
    using Game;
    using Util;
    using System.Collections.Generic;

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
        #endregion

        #region Static Fields
        private static MainManager instance;
        #endregion

        #region Properties
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

        public static void LoadScene(SceneType sceneType)
        {
            string sceneName = Enum.GetName(typeof(SceneType), sceneType);
            Debug.Log($"[MainManager] Loading Scene '{sceneName}'.");
            switch (sceneType)
            {
                case SceneType.MainMenu:
                    Instance.StartCoroutine(FindObjectOfType<UI.Screen.LoadingScreen>().LoadSceneAsynchronously(sceneName, () =>
                    {
                        InitializeScene("MainMenu");
                    }));
                    break;
                case SceneType.Game:
                    Instance.StartCoroutine(FindObjectOfType<UI.Screen.LoadingScreen>().LoadSceneAsynchronously(sceneName, () =>
                    {
                        InitializeScene("Game");
                    }));
                    break;
                default:
                    throw new ArgumentException($"Scene Type '{sceneName}' not supported!");
            }
            Debug.Log($"[MainManager] Finished loading Scene '{sceneName}'.");
        }

        public static void CreateNewGame()
        {
            LoadScene(SceneType.Game);
            /*
            Universe.GenerationSettings generationSettings = new Universe.GenerationSettings();

            #region Universe Generation Settings Default
            generationSettings.Seed = 0;
            generationSettings.Size = 2048;
            generationSettings.MapFromMin = -1.0f;
            generationSettings.MapFromMax = 1.0f;
            generationSettings.MapToMin = 0.0f;
            generationSettings.MapToMax = 1.0f;
            generationSettings.Power = 5.0f;
            generationSettings.Amplitude = 64.0f;

            generationSettings.NoiseType = FastNoiseLite.NoiseType.Cellular;
            generationSettings.Frequency = 0.04f;

            generationSettings.FractalType = FastNoiseLite.FractalType.FBm;
            generationSettings.FractalOctaves = 3;
            generationSettings.FractalLacunarity = 2.0f;
            generationSettings.FractalGain = 0.5f;
            generationSettings.FractalWeightedStrength = 1.0f;

            generationSettings.CellularDistanceFunction = FastNoiseLite.CellularDistanceFunction.EuclideanSq;
            generationSettings.CellularReturnType = FastNoiseLite.CellularReturnType.Distance;
            generationSettings.CellularJitter = 1.0f;

            generationSettings.DomainWarpType = FastNoiseLite.DomainWarpType.OpenSimplex2;
            generationSettings.DomainWarpAmplitude = 20.0f;
            generationSettings.DomainWarpFrequency = 0.01f;

            generationSettings.DomainWarpFractalType = FastNoiseLite.FractalType.DomainWarpProgressive;
            generationSettings.DomainWarpFractalOctaves = 5;
            generationSettings.DomainWarpFractalLacunarity = 2.0f;
            generationSettings.DomainWarpFractalGain = 0.5f;
            #endregion

            #region Filament Generation Settings Default
            generationSettings.FilamentGenerationSettings.Size = 2048;
            generationSettings.FilamentGenerationSettings.MapFromMin = -1.0f;
            generationSettings.FilamentGenerationSettings.MapFromMax = 1.0f;
            generationSettings.FilamentGenerationSettings.MapToMin = 0.0f;
            generationSettings.FilamentGenerationSettings.MapToMax = 1.0f;
            generationSettings.FilamentGenerationSettings.UniverseNoiseInfluence = 1.0f;
            generationSettings.FilamentGenerationSettings.Power = 1.0f;
            generationSettings.FilamentGenerationSettings.Amplitude = 1.0f;

            generationSettings.FilamentGenerationSettings.NoiseType = FastNoiseLite.NoiseType.Cellular;
            generationSettings.FilamentGenerationSettings.Frequency = 0.02f;

            generationSettings.FilamentGenerationSettings.FractalType = FastNoiseLite.FractalType.FBm;
            generationSettings.FilamentGenerationSettings.FractalOctaves = 5;
            generationSettings.FilamentGenerationSettings.FractalLacunarity = 2.0f;
            generationSettings.FilamentGenerationSettings.FractalGain = 0.5f;
            generationSettings.FilamentGenerationSettings.FractalWeightedStrength = 0.0f;

            generationSettings.FilamentGenerationSettings.CellularDistanceFunction = FastNoiseLite.CellularDistanceFunction.EuclideanSq;
            generationSettings.FilamentGenerationSettings.CellularReturnType = FastNoiseLite.CellularReturnType.Distance;
            generationSettings.FilamentGenerationSettings.CellularJitter = 1.0f;

            generationSettings.FilamentGenerationSettings.DomainWarpType = FastNoiseLite.DomainWarpType.OpenSimplex2;
            generationSettings.FilamentGenerationSettings.DomainWarpAmplitude = 20.0f;
            generationSettings.FilamentGenerationSettings.DomainWarpFrequency = 0.005f;

            generationSettings.FilamentGenerationSettings.DomainWarpFractalType = FastNoiseLite.FractalType.DomainWarpProgressive;
            generationSettings.FilamentGenerationSettings.DomainWarpFractalOctaves = 5;
            generationSettings.FilamentGenerationSettings.DomainWarpFractalLacunarity = 2.0f;
            generationSettings.FilamentGenerationSettings.DomainWarpFractalGain = 0.5f;
            #endregion

            #region Sectror Generation Settings Default
            generationSettings.SectorGenerationSettings.Size = 2048;
            generationSettings.SectorGenerationSettings.MapFromMin = -1.0f;
            generationSettings.SectorGenerationSettings.MapFromMax = 1.0f;
            generationSettings.SectorGenerationSettings.MapToMin = 0.0f;
            generationSettings.SectorGenerationSettings.MapToMax = 1.0f;
            generationSettings.SectorGenerationSettings.FilamentNoiseInfluence = 1.0f;
            generationSettings.SectorGenerationSettings.Power = 1.0f;
            generationSettings.SectorGenerationSettings.Amplitude = 1.0f;

            generationSettings.SectorGenerationSettings.NoiseType = FastNoiseLite.NoiseType.OpenSimplex2;
            generationSettings.SectorGenerationSettings.Frequency = 0.01f;

            generationSettings.SectorGenerationSettings.FractalType = FastNoiseLite.FractalType.FBm;
            generationSettings.SectorGenerationSettings.FractalOctaves = 5;
            generationSettings.SectorGenerationSettings.FractalLacunarity = 2.0f;
            generationSettings.SectorGenerationSettings.FractalGain = 0.5f;
            generationSettings.SectorGenerationSettings.FractalWeightedStrength = 0.0f;

            generationSettings.SectorGenerationSettings.DomainWarpType = FastNoiseLite.DomainWarpType.OpenSimplex2;
            generationSettings.SectorGenerationSettings.DomainWarpAmplitude = 20.0f;
            generationSettings.SectorGenerationSettings.DomainWarpFrequency = 0.005f;

            generationSettings.SectorGenerationSettings.DomainWarpFractalType = FastNoiseLite.FractalType.DomainWarpProgressive;
            generationSettings.SectorGenerationSettings.DomainWarpFractalOctaves = 5;
            generationSettings.SectorGenerationSettings.DomainWarpFractalLacunarity = 2.0f;
            generationSettings.SectorGenerationSettings.DomainWarpFractalGain = 0.5f;
            #endregion

            #region Region Generation Settings Default
            generationSettings.RegionGenerationSettings.Size = 2048;
            generationSettings.RegionGenerationSettings.MapFromMin = -1.0f;
            generationSettings.RegionGenerationSettings.MapFromMax = 1.0f;
            generationSettings.RegionGenerationSettings.MapToMin = -0.375f;
            generationSettings.RegionGenerationSettings.MapToMax = 1.375f;
            generationSettings.RegionGenerationSettings.SectorNoiseInfluence = 1.0f;
            generationSettings.RegionGenerationSettings.Power = 1.0f;
            generationSettings.RegionGenerationSettings.Amplitude = 1.0f;

            generationSettings.RegionGenerationSettings.NoiseType = FastNoiseLite.NoiseType.OpenSimplex2;
            generationSettings.RegionGenerationSettings.Frequency = 0.005f;

            generationSettings.RegionGenerationSettings.FractalType = FastNoiseLite.FractalType.FBm;
            generationSettings.RegionGenerationSettings.FractalOctaves = 5;
            generationSettings.RegionGenerationSettings.FractalLacunarity = 2.0f;
            generationSettings.RegionGenerationSettings.FractalGain = 0.5f;
            generationSettings.RegionGenerationSettings.FractalWeightedStrength = 0.0f;

            generationSettings.RegionGenerationSettings.DomainWarpType = FastNoiseLite.DomainWarpType.OpenSimplex2;
            generationSettings.RegionGenerationSettings.DomainWarpAmplitude = 20.0f;
            generationSettings.RegionGenerationSettings.DomainWarpFrequency = 0.005f;

            generationSettings.RegionGenerationSettings.DomainWarpFractalType = FastNoiseLite.FractalType.DomainWarpProgressive;
            generationSettings.RegionGenerationSettings.DomainWarpFractalOctaves = 5;
            generationSettings.RegionGenerationSettings.DomainWarpFractalLacunarity = 2.0f;
            generationSettings.RegionGenerationSettings.DomainWarpFractalGain = 0.5f;
            #endregion

            Vector2Int filamentPosition = new Vector2Int(1024, 1024);
            Vector2Int sectorPosition = new Vector2Int(1024, 1024);
            Vector2Int regionPosition = new Vector2Int(1024, 1024);
            Universe.GenerateUniverse(generationSettings);
            Universe.Instance.GenerateFilament(filamentPosition);
            Universe.Instance.GenerateSector(filamentPosition, sectorPosition);
            Universe.Instance.GenerateRegion(sectorPosition, regionPosition);
            */
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
