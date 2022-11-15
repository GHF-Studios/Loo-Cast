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

    public class GameManager : MonoBehaviour
    {
        #region Static Properties
        public static GameManager Instance { get; private set; }
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
        }
        #endregion

        #region Fields
        public LoadingScreen loadingScreen;
        public RuntimeSets runtimeSets;
        public GameSoundHandler gameSoundHandler;

        private Game currentGame;
        #endregion

        #region Unity Callbacks
        private void Awake()
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

        private void OnApplicationQuit()
        {
            runtimeSets.Initialize();
        }
        #endregion

        #region Static Methods
        public static void Pause()
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

        public static void Resume()
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

        public static void TogglePause()
        {
            if (Instance == null)
            {
                return;
            }
            if (Instance.IsPaused)
            {
                Resume();
            }
            else
            {
                Pause();
            }
        }

        public static void LoadScene(string sceneIndex)
        {
            if (Instance == null)
            {
                return;
            }
            Instance.StartCoroutine(Instance.loadingScreen.LoadSceneAsynchronously(sceneIndex));
        }

        public static void LoadGame()
        {

        }
        #endregion
    }
}
