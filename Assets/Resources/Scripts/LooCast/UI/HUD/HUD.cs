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

                bool stationRaycastSuccess = Physics.Raycast(ray, out hit, Mathf.Infinity, stationScreenLayerMask, QueryTriggerInteraction.Collide);
                if (stationRaycastSuccess)
                {
                    AllyStation allyStation = hit.transform.gameObject.GetComponentInParent<AllyStation>();
                    if (allyStation != null)
                    {
                        stationScreen.CurrentAllyStation = allyStation;
                        stationScreen.SetVisibility(true);
                    }
                }
            }
            else if (Input.GetMouseButtonDown(0))
            {
                RaycastHit hit;
                Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
                
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
