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
            public GameObject prefab;
            // How many Sectors fit into a Filament (Per Axis)
            public int size;
        }
        #endregion

        public Vector2Int WorldPosition => worldPosition;
        public Vector2Int FilamentPosition => filamentPosition;

        [SerializeField] private Vector2Int filamentPosition;
        [SerializeField] private Vector2Int worldPosition;

        private GameObject filamentObject;

        public Filament(Vector2Int filamentPosition)
        {
            this.filamentPosition = filamentPosition;
            worldPosition = filamentPosition * Universe.Instance.FilamentGenerationSettings.size;
        }

        public void Spawn()
        {
            filamentObject = GameObject.Instantiate(Universe.Instance.FilamentGenerationSettings.prefab);
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