using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Inspector
{
    using Data.Runtime;
    using LooCast.UI.Panel;
    using LooCast.Asteroid;
    
    public class AsteroidInspector : MonoBehaviour
    {
        [SerializeField] private AsteroidInspectorRuntimeData runtimeData;
        [SerializeField] private AsteroidInfoPanel asteroidInfoPanel;
        [SerializeField] private AsteroidResourceDepositsPanel asteroidResourceDepositsPanel;

        private void Start()
        {
            runtimeData.OnCurrentAsteroidChanged.AddListener(() => 
            { 
                if (runtimeData.CurrentAsteroid == null)
                {
                    gameObject.SetActive(false);
                }
                else
                {
                    gameObject.SetActive(true);
                    asteroidInfoPanel.CurrentAsteroid = runtimeData.CurrentAsteroid; 
                    asteroidResourceDepositsPanel.CurrentAsteroid = runtimeData.CurrentAsteroid;
                }
            });

            if (runtimeData.CurrentAsteroid == null)
            {
                gameObject.SetActive(false);
            }
        }
    }
}