using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Manager
{
    using Data.Runtime;
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
        public void Pause()
        {
            if (!IsPaused)
            {
                IsPaused = true;
                foreach (ExtendedMonoBehaviour extendedMonoBehaviour in ExtendedMonoBehaviour.Instances)
                {
                    extendedMonoBehaviour.Pause();
                }
            }
        }

        public void Resume()
        {
            if (IsPaused)
            {
                IsPaused = false;
                foreach (ExtendedMonoBehaviour extendedMonoBehaviour in ExtendedMonoBehaviour.Instances)
                {
                    extendedMonoBehaviour.Resume();
                }
            }
        }

        public void TogglePause()
        {
            if (IsPaused)
            {
                Resume();
            }
            else
            {
                Pause();
            }
        }

        public void LoadScene(string sceneIndex)
        {
            StartCoroutine(loadingScreen.LoadSceneAsynchronously(sceneIndex));
        }
        #endregion
    }
}
