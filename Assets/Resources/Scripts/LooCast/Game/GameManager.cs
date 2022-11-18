using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Game
{
    using LooCast.Data.Runtime;
    using LooCast.UI.Screen;
    using LooCast.Sound;
    using LooCast.Core;
    using LooCast.Statistic;
    using System;
    using LooCast.Util;

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
