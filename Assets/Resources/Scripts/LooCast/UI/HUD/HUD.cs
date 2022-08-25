using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.HUD
{
    using LooCast.Asteroid;
    using LooCast.UI.Inspector.Data.Runtime;
    using LooCast.UI.Cursor;

    public class HUD : MonoBehaviour
    {
        #region Fields
        [SerializeField] private AsteroidInspectorRuntimeData asteroidInspectorRuntimeData;
        [SerializeField] private LayerMask asteroidCursorLayerMask;
        [SerializeField] private AsteroidCursor asteroidCursor;
        #endregion

        #region Unity Callbacks
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
                    hitAsteroid.Destroy();
                }
            }
        }
        #endregion
    }
}
