using UnityEngine;
using System;

namespace LooCast.Region
{
    using LooCast.Universe;
    using LooCast.Noise;
    using LooCast.Util;
    using LooCast.Test;

    public class Region
    {
        #region Structs
        [Serializable]
        private struct NoiseMap
        {
            public float[,] DataPointArray2D
            {
                get
                {
                    float[,] dataPointArray2D = new float[arrayWidth, arrayHeight];
                    for (int y = 0; y < arrayHeight; y++)
                    {
                        for (int x = 0; x < arrayWidth; x++)
                        {
                            dataPointArray2D[x, y] = dataPointArray1D[y * arrayWidth + x];
                        }
                    }
                    return dataPointArray2D;
                }
            }

            [SerializeField] private float[] dataPointArray1D;
            [SerializeField] private int arrayWidth;
            [SerializeField] private int arrayHeight;

            public NoiseMap(float[,] dataPointArray2D)
            {
                dataPointArray1D = new float[dataPointArray2D.Length];
                arrayWidth = dataPointArray2D.GetLength(0);
                arrayHeight = dataPointArray2D.GetLength(1);

                for (int y = 0; y < arrayHeight; y++)
                {
                    for (int x = 0; x < arrayWidth; x++)
                    {
                        dataPointArray1D[y * arrayWidth + x] = dataPointArray2D[x, y];
                    }
                }
            }
        }
        #endregion

        public Vector2 WorldPosition => worldPosition;
        public Vector2Int RegionPosition => regionPosition;

        [SerializeField] private int size;
        [SerializeField] private Vector2Int regionPosition;
        [SerializeField] private Vector2 worldPosition;

        [SerializeField] private NoiseMap noiseMap;

        private GameObject regionObject;

        public Region(Vector2Int regionPosition, int size, Universe.GenerationSettings generationSettings)
        {
            this.size = size;
            this.regionPosition = regionPosition;
            worldPosition = regionPosition * size;

            //Any world generation happens here
            noiseMap = new NoiseMap(PerlinNoise.GenerateNoiseMap
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
            ));
        }

        public void Spawn(GameObject prefab)
        {
            regionObject = GameObject.Instantiate(prefab);
            regionObject.name = $"Region ({regionPosition.x}, {regionPosition.y})";
            regionObject.transform.position = worldPosition * 10.0f;

            MapDisplay mapDisplay = regionObject.GetComponentInChildren<MapDisplay>();
            mapDisplay.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap.DataPointArray2D));
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(regionObject);
        }
    }
}