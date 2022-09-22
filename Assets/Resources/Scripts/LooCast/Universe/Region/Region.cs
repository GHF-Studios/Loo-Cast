using UnityEngine;
using System;

namespace LooCast.Universe.Region
{
    using Filament;
    using LooCast.Noise;
    using LooCast.Util;
    using LooCast.Test;
    using LooCast.Math.Map;

    public class Region
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            public GameObject prefab;
            // How many Units fit into a Region (Per Axis)
            public int size;
            public float scale;
            public float amplitude;
            public int octaves;
            public float persistence;
            public float lacunarity;
        }
        #endregion

        public Vector2Int WorldPosition => worldPosition;
        public Vector2Int RegionPosition => regionPosition;

        [SerializeField] private Vector2Int sectorPosition;
        [SerializeField] private Vector2Int regionPosition;
        [SerializeField] private Vector2Int worldPosition;

        [SerializeField] private FloatMap2D noiseMap;

        private GameObject regionObject;

        public Region(Vector2Int sectorPosition, Vector2Int regionPosition)
        {
            this.sectorPosition = sectorPosition;
            this.regionPosition = regionPosition;
            worldPosition = regionPosition * Universe.Instance.RegionGenerationSettings.size;

            //Any world generation happens here
            noiseMap = PerlinNoise.GenerateNoiseMap
            (
                Universe.Instance.RegionGenerationSettings.size, 
                Universe.Instance.RegionGenerationSettings.size, 
                Universe.Instance.UniverseGenerationSettings.seed,
                Universe.Instance.RegionGenerationSettings.scale, 
                Universe.Instance.RegionGenerationSettings.octaves, 
                Universe.Instance.RegionGenerationSettings.persistence, 
                Universe.Instance.RegionGenerationSettings.lacunarity, 
                Universe.Instance.RegionGenerationSettings.amplitude, 
                -worldPosition
            );
        }

        public void Spawn()
        {
            regionObject = GameObject.Instantiate(Universe.Instance.RegionGenerationSettings.prefab);
            regionObject.name = $"Region ({regionPosition.x}, {regionPosition.y})";
            regionObject.transform.position = new Vector3(worldPosition.x, worldPosition.y, 0.0f) * 10.0f;

            MapDisplay mapDisplay = regionObject.GetComponentInChildren<MapDisplay>();
            mapDisplay.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap.Array2D));
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(regionObject);
        }
    }
}