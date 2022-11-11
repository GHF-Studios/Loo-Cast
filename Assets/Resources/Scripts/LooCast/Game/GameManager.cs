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

    public class GameManager : MonoBehaviour
    {
        #region Properties
        public static GameManager Instance { get; private set; }
        public bool IsPaused { get; private set; }
        #endregion

        #region Fields
        public LoadingScreen loadingScreen;
        public RuntimeSets runtimeSets;
        public GameSoundHandler gameSoundHandler;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            if (Instance != null && Instance != this)
            {
                Destroy(gameObject);
            }
            else
            {
                Instance = this;
            }

            runtimeSets.Initialize();
            IsPaused = false;
            KillsStatistic.Kills = 0;
        }

        private void OnApplicationQuit()
        {
            runtimeSets.Initialize();
        }
        #endregion

        #region Methods
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
        #endregion
    }
}
