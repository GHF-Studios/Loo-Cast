using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Manager
{
    using Data.Runtime;
    using LooCast.UI.Screen;
    using LooCast.Sound;
    using LooCast.Core;

    public class GameManager : MonoBehaviour
    {
        #region Properties
        public static GameManager Instance { get; private set; }
        public bool IsPaused { get; private set; }
        #endregion

        #region Fields
        public LoadingScreen LoadingScreen;
        public RuntimeSets RuntimeSets;
        public GameSoundHandler GameSoundHandler;
        #endregion

        #region Methods
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

            RuntimeSets.Initialize();
            IsPaused = false;
        }

        private void OnApplicationQuit()
        {
            RuntimeSets.Initialize();
        }

        public void Pause()
        {
            if (!IsPaused)
            {
                IsPaused = true;
                foreach (ExtendedMonoBehaviour extendedMonoBehaviour in RuntimeSets.ExtendedMonoBehaviourRuntimeSet.Items)
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
                foreach (ExtendedMonoBehaviour extendedMonoBehaviour in RuntimeSets.ExtendedMonoBehaviourRuntimeSet.Items)
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
            StartCoroutine(LoadingScreen.LoadSceneAsynchronously(sceneIndex));
        }
        #endregion
    }
}
