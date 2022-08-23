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
    using LooCast.Asteroid;
    using LooCast.UI.Inspector.Data.Runtime;
    using LooCast.UI.Cursor;

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

        [SerializeField] private AsteroidInspectorRuntimeData asteroidInspectorRuntimeData;
        [SerializeField] private LayerMask asteroidCursorLayerMask;
        [SerializeField] private AsteroidCursor asteroidCursor;
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

            runtimeSets.Initialize();
            IsPaused = false;
            KillsStatistic.Kills = 0;
        }

        private void Update()
        {
            if (Input.GetMouseButtonDown(1))
            {
                RaycastHit hit;
                Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
    
                if (Physics.Raycast(ray, out hit, Mathf.Infinity, asteroidCursorLayerMask))
                {
                    Asteroid hitAsteroid = hit.transform.gameObject.GetComponent<Asteroid>();
                    asteroidInspectorRuntimeData.CurrentAsteroid = hitAsteroid;
                    asteroidCursor.CurrentAsteroid = hitAsteroid;
                }
                else
                {
                    asteroidInspectorRuntimeData.CurrentAsteroid = null;
                    asteroidCursor.CurrentAsteroid = null;
                }
            }
            else if (Input.GetMouseButtonDown(0))
            {
                RaycastHit hit;
                Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);

                if (Physics.Raycast(ray, out hit, Mathf.Infinity, asteroidCursorLayerMask))
                {
                    Asteroid hitAsteroid = hit.transform.gameObject.GetComponent<Asteroid>();
                    if (hitAsteroid == asteroidCursor.CurrentAsteroid)
                    {
                        asteroidInspectorRuntimeData.CurrentAsteroid = null;
                        asteroidCursor.CurrentAsteroid = null;
                    }
                    Destroy(hitAsteroid.gameObject);
                }
            }
        }

        private void OnApplicationQuit()
        {
            runtimeSets.Initialize();
        }

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
