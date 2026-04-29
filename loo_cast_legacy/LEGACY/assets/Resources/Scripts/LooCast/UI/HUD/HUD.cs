using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.HUD
{
    using LooCast.Asteroid;
    using LooCast.Station;
    using LooCast.UI.Inspector.Data.Runtime;
    using LooCast.UI.Cursor;
    using LooCast.UI.Screen;

    public class HUD : MonoBehaviour
    {
        #region Fields
        [SerializeField] private AsteroidInspectorRuntimeData asteroidInspectorRuntimeData;
        [SerializeField] private AsteroidCursor asteroidCursor;
        [SerializeField] private StationScreen stationScreen;
        [SerializeField] private LayerMask worldLayerMask;
        #endregion

        #region Unity Callbacks
        private void Update()
        {
            #region LMB Input Handler
            if (Input.GetMouseButtonDown(0))
            {
                Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
                RaycastHit[] hits = Physics.RaycastAll(ray, Mathf.Infinity, worldLayerMask, QueryTriggerInteraction.Collide);

                #region Asteroid Destruction
                if (hits != null && hits.Length > 0)
                {
                    Asteroid hitAsteroid = null;
                    foreach (RaycastHit hit in hits)
                    {
                        hitAsteroid = hit.transform.gameObject.GetComponent<Asteroid>();
                        if (hitAsteroid != null)
                        {
                            break;
                        }
                    }

                    if (hitAsteroid == asteroidCursor.CurrentAsteroid)
                    {
                        asteroidInspectorRuntimeData.CurrentAsteroid = null;
                        asteroidCursor.CurrentAsteroid = null;
                    }
                    hitAsteroid.Destroy();
                }
                #endregion
            }
            #endregion

            #region RMB Input Handler
            else if (Input.GetMouseButtonDown(1))
            {
                Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
                RaycastHit[] hits = Physics.RaycastAll(ray, Mathf.Infinity, worldLayerMask, QueryTriggerInteraction.Collide);

                #region Asteroid Inspector
                if (hits != null && hits.Length > 0)
                {
                    Asteroid hitAsteroid = null;
                    foreach (RaycastHit hit in hits)
                    {
                        hitAsteroid = hit.transform.gameObject.GetComponent<Asteroid>();
                        if (hitAsteroid != null)
                        {
                            break;
                        }
                    }

                    if (hitAsteroid != null)
                    {
                        asteroidInspectorRuntimeData.CurrentAsteroid = hitAsteroid;
                        asteroidCursor.CurrentAsteroid = hitAsteroid;
                    }
                }
                else
                {
                    asteroidInspectorRuntimeData.CurrentAsteroid = null;
                    asteroidCursor.CurrentAsteroid = null;
                }
                #endregion

                #region Station Screen
                if (hits != null && hits.Length > 0)
                {
                    AllyStation hitAllyStation = null;
                    foreach (RaycastHit hit in hits)
                    {
                        hitAllyStation = hit.transform.gameObject.GetComponentInParent<AllyStation>();
                        if (hitAllyStation != null)
                        {
                            break;
                        }
                    }

                    if (hitAllyStation != null)
                    {
                        stationScreen.CurrentAllyStation = hitAllyStation;
                        stationScreen.SetVisibility(true);
                    }
                }
                #endregion
            }
            #endregion
        }
        #endregion
    }
}
