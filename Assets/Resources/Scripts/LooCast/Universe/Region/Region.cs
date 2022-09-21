using UnityEngine;
using System;

namespace LooCast.Universe.Region
{
    using LooCast.Noise;
    using LooCast.Util;
    using LooCast.Test;
    using LooCast.Math.Map;

    public class Region
    {
        public Vector2 WorldPosition => worldPosition;
        public Vector2Int RegionPosition => regionPosition;

        [SerializeField] private int size;
        [SerializeField] private Vector2Int regionPosition;
        [SerializeField] private Vector2 worldPosition;

        [SerializeField] private FloatMap2D noiseMap;

        private GameObject regionObject;

        public Region(Vector2Int regionPosition, int size, Universe.GenerationSettings generationSettings)
        {
            this.size = size;
            this.regionPosition = regionPosition;
            worldPosition = regionPosition * size;

            //Any world generation happens here
            noiseMap = PerlinNoise.GenerateNoiseMap
            (
                size, 
                size, 
                generationSettings.seed,
                generationSettings.scale, 
                generationSettings.octaves, 
                generationSettings.persistence, 
                generationSettings.lacunarity, 
                generationSettings.amplitude, 
                -worldPosition
            );
        }

        public void Spawn(GameObject prefab)
        {
            regionObject = GameObject.Instantiate(prefab);
            regionObject.name = $"Region ({regionPosition.x}, {regionPosition.y})";
            regionObject.transform.position = worldPosition * 10.0f;

            MapDisplay mapDisplay = regionObject.GetComponentInChildren<MapDisplay>();
            mapDisplay.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap.Array2D));
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(regionObject);
        }
    }
}