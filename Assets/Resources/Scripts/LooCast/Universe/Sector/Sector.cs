using UnityEngine;
using System;

namespace LooCast.Universe.Sector
{
    using LooCast.Test;
    using LooCast.Util;

    public class Sector
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            // How many Regions fit into a Sector (Per Axis)
            public int size;
            public GameObject prefab;
        }
        #endregion

        public Vector2Int WorldPosition => worldPosition;
        public Vector2Int SectorPosition => sectorPosition;

        [SerializeField] private Universe.GenerationSettings universeGenerationSettings;
        [SerializeField] private GenerationSettings generationSettings;
        [SerializeField] private Vector2Int filamentPosition;
        [SerializeField] private Vector2Int sectorPosition;
        [SerializeField] private Vector2Int worldPosition;

        private GameObject sectorObject;

        public Sector(Universe.GenerationSettings universeGenerationSettings, Vector2Int filamentPosition, Vector2Int sectorPosition)
        {
            this.universeGenerationSettings = universeGenerationSettings;
            generationSettings = universeGenerationSettings.sectorGenerationSettings;
            this.filamentPosition = filamentPosition;
            this.sectorPosition = sectorPosition;
            worldPosition = sectorPosition * generationSettings.size;
        }

        public void Spawn()
        {
            sectorObject = GameObject.Instantiate(generationSettings.prefab);
            sectorObject.name = $"Sector ({sectorPosition.x}, {sectorPosition.y})";
            sectorObject.transform.position = new Vector3(worldPosition.x, worldPosition.y, 0.0f) * 10.0f;

            MapDisplay mapDisplay = sectorObject.GetComponentInChildren<MapDisplay>();
            //mapDisplay.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap.DataPointArray2D));
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(sectorObject);
        }
    }
}