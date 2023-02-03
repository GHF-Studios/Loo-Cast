using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Cursor
{
    using LooCast.Asteroid;

    public class AsteroidCursor : MonoBehaviour
    {
        public Asteroid CurrentAsteroid
        {
            get
            {
                return currentAsteroid;
            }
            set
            {
                currentAsteroid = value;
                if (currentAsteroid == null)
                {
                    gameObject.SetActive(false);
                }
                else
                {
                    Refresh();
                    gameObject.SetActive(true);
                }
            }
        }
        private Asteroid currentAsteroid;

        private void Update()
        {
            Refresh();
        }

        public void Refresh()
        {
            transform.position = new Vector3(currentAsteroid.transform.position.x, currentAsteroid.transform.position.y, transform.position.z);
            Bounds currentAsteroidBounds = currentAsteroid.GetComponent<MeshRenderer>().bounds;
            float furthestExtent = currentAsteroidBounds.extents.x;
            if (currentAsteroidBounds.extents.y > furthestExtent)
            {
                furthestExtent = currentAsteroidBounds.extents.y;
            }
            if (currentAsteroidBounds.extents.z > furthestExtent)
            {
                furthestExtent = currentAsteroidBounds.extents.z;
            }
            transform.localScale = Vector3.one * furthestExtent / 20.0f;
        }
    }
}