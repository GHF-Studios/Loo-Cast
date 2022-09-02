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
        [SerializeField] private LayerMask asteroidCursorLayerMask;
        [SerializeField] private AsteroidCursor asteroidCursor;

        [SerializeField] private LayerMask stationScreenLayerMask;
        [SerializeField] private StationScreen stationScreen;
        #endregion

        #region Unity Callbacks
        private void Update()
        {
            if (Input.GetMouseButtonDown(1))
            {
                RaycastHit hit;
                Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);

                //Asteroid
                bool asteroidRaycastSuccess = Physics.Raycast(ray, out hit, Mathf.Infinity, asteroidCursorLayerMask);
                if (asteroidRaycastSuccess)
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

                //Station
                RaycastHit2D[] stationRaycastHits = Physics2D.GetRayIntersectionAll(ray, float.PositiveInfinity, stationScreenLayerMask);
                if (stationRaycastHits != null && stationRaycastHits.Length > 0)
                {
                    foreach (RaycastHit2D raycastHit2D in stationRaycastHits)
                    {
                        if (raycastHit2D.collider)
                        {
                            PlayerStation hitPlayerStation = stationRaycastHits[0].collider.gameObject.GetComponent<PlayerStation>();
                            stationScreen.CurrentPlayerStation = hitPlayerStation;
                            stationScreen.SetVisibility(true);
                            break;
                        }
                    }
                    
                }
            }
            else if (Input.GetMouseButtonDown(0))
            {
                RaycastHit hit;
                Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
                
                //Asteroid
                bool asteroidRaycastSuccess = Physics.Raycast(ray, out hit, Mathf.Infinity, asteroidCursorLayerMask);
                if (asteroidRaycastSuccess)
                {
                    Asteroid hitAsteroid = hit.transform.gameObject.GetComponent<Asteroid>();
                    if (hitAsteroid == asteroidCursor.CurrentAsteroid)
                    {
                        asteroidInspectorRuntimeData.CurrentAsteroid = null;
                        asteroidCursor.CurrentAsteroid = null;
                    }
                    hitAsteroid.Destroy();
                }
            }
        }
        #endregion
    }
}
