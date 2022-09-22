using UnityEngine;
using System;

namespace LooCast.Universe.Filament
{
    using LooCast.Test;
    using LooCast.Util;

    public class Filament
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            // How many Sectors fit into a Filament (Per Axis)
            public int size;
            public GameObject prefab;
        }
        #endregion

        public Vector2Int WorldPosition => worldPosition;
        public Vector2Int FilamentPosition => filamentPosition;

        [SerializeField] private Universe.GenerationSettings universeGenerationSettings;
        [SerializeField] private GenerationSettings generationSettings;
        [SerializeField] private Vector2Int filamentPosition;
        [SerializeField] private Vector2Int worldPosition;

        private GameObject filamentObject;

        public Filament(Universe.GenerationSettings universeGenerationSettings, Vector2Int filamentPosition)
        {
            this.universeGenerationSettings = universeGenerationSettings;
            generationSettings = universeGenerationSettings.filamentGenerationSettings;
            this.filamentPosition = filamentPosition;
            worldPosition = filamentPosition * generationSettings.size;
        }

        public void Spawn()
        {
            filamentObject = GameObject.Instantiate(generationSettings.prefab);
            filamentObject.name = $"Filament ({filamentPosition.x}, {filamentPosition.y})";
            filamentObject.transform.position = new Vector3(worldPosition.x, worldPosition.y, 0.0f) * 10.0f;

            MapDisplay mapDisplay = filamentObject.GetComponentInChildren<MapDisplay>();
            //mapDisplay.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap.DataPointArray2D));
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(filamentObject);
        }
    }
}