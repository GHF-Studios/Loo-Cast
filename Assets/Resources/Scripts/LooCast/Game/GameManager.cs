using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Game
{
    using Data.Runtime;
    using UI.Screen;
    using Sound;
    using Core;
    using Statistic;
    using System;
    using Util;
    using Universe;

    public class GameManager : MonoBehaviour
    {
        #region Static Properties
        public static GameManager Instance { get; private set; }
        public static bool Initialized
        {
            get
            {
                return Instance != null;
            }
        }
        #endregion

        #region Static Fields
        #endregion

        #region Properties
        public bool IsPaused { get; private set; }
        public Game CurrentGame
        {
            get
            {
                return currentGame;
            }

            private set
            {
                currentGame = value;
            }
        }
        #endregion

        #region Fields
        public LoadingScreen loadingScreen;
        public RuntimeSets runtimeSets;
        public GameSoundHandler gameSoundHandler;

        private Game currentGame;
        #endregion

        #region Unity Callbacks
        private void OnApplicationQuit()
        {
            runtimeSets.Initialize();
        }
        #endregion

        #region Methods
        public void Initialize()
        {
            if (Instance != null)
            {
                throw new Exception("Cannot have multiple instances of GameManager!");
            }

            #region Initialization
            Instance = this;
            runtimeSets.Initialize();
            IsPaused = false;
            KillsStatistic.Kills = 0;
            currentGame = null;

            #region Universe Pre-Generation
            Universe.GenerationSettings generationSettings = new Universe.GenerationSettings();

            #region Universe Generation Settings Default
            generationSettings.Seed = 0;
            generationSettings.Size = 4096;
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
            generationSettings.FilamentGenerationSettings.ChunkSize = 64;
            generationSettings.FilamentGenerationSettings.ChunkAmount = 64;
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
            generationSettings.SectorGenerationSettings.ChunkSize = 64;
            generationSettings.SectorGenerationSettings.ChunkAmount = 64;
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
            generationSettings.RegionGenerationSettings.ChunkSize = 64;
            generationSettings.RegionGenerationSettings.ChunkAmount = 64;
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

            currentGame.GenerateUniverse(generationSettings);
            #endregion

            #endregion

            Debug.Log($"[GameManager] Initialized.");
        }
        #endregion

        #region Static Methods
        public static void PauseGame()
        {
            if (Instance == null)
            {
                return;
            }
            if (!Instance.IsPaused)
            {
                Instance.IsPaused = true;
                foreach (ExtendedMonoBehaviour extendedMonoBehaviour in ExtendedMonoBehaviour.Instances)
                {
                    extendedMonoBehaviour.Pause();
                }
            }
        }

        public static void ResumeGame()
        {
            if (Instance == null)
            {
                return;
            }
            if (Instance.IsPaused)
            {
                Instance.IsPaused = false;
                foreach (ExtendedMonoBehaviour extendedMonoBehaviour in ExtendedMonoBehaviour.Instances)
                {
                    extendedMonoBehaviour.Resume();
                }
            }
        }

        public static void LoadGame(Game game)
        {
            if (Instance == null)
            {
                return;
            }
            Instance.currentGame = game;

            // Load all Chunks inside range at player position into Scene

        }

        public static void SaveGame(Game game)
        {
            if (Instance == null)
            {
                return;
            }
            // Save all loaded Chunks
        }
        #endregion
    }
}
