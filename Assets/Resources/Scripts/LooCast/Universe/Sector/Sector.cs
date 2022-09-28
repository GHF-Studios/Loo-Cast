using UnityEngine;
using System;

namespace LooCast.Universe.Sector
{
    using LooCast.Test;
    using LooCast.Util;

    [Serializable]
    public class Sector
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            public GameObject prefab;
            public int size;
        }
        #endregion

        public Vector2Int WorldPosition => worldPosition;
        public Vector2Int SectorPosition => sectorPosition;
        public Texture2D Map
        {
            get
            {
                return map;
            }

            set
            {
                map = value;
            }
        }

        [SerializeField] private Vector2Int filamentPosition;
        [SerializeField] private Vector2Int sectorPosition;
        [SerializeField] private Vector2Int worldPosition;

        private GameObject sectorObject;
        private Texture2D map;

        public Sector(Vector2Int filamentPosition, Vector2Int sectorPosition)
        {
            this.filamentPosition = filamentPosition;
            this.sectorPosition = sectorPosition;
            worldPosition = sectorPosition * Universe.Instance.SectorGenerationSettings.size;
        }

        public void Spawn()
        {
            sectorObject = GameObject.Instantiate(Universe.Instance.SectorGenerationSettings.prefab);
            sectorObject.name = $"Sector ({sectorPosition.x}, {sectorPosition.y})";
            sectorObject.transform.position = new Vector3(worldPosition.x, worldPosition.y, 0.0f) * 10.0f;

            MapDisplay mapDisplay = sectorObject.GetComponentInChildren<MapDisplay>();
            mapDisplay.DrawTexture(map);
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(sectorObject);
        }
    }
}