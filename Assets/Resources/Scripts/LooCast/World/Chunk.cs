using UnityEngine;
using System;

namespace LooCast.World
{
    using LooCast.Noise;

    public class Chunk
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

        #region Enums
        public enum State
        {
            Loaded,
            Spawned
        }
        #endregion

        public Vector2 WorldPosition => worldPosition;
        public Vector2Int ChunkPosition => chunkPosition;

        [SerializeField] private int size;
        [SerializeField] private Vector2Int chunkPosition;
        [SerializeField] private Vector2 worldPosition;

        [SerializeField] private NoiseMap noiseMap; //TODO: Make serializeable

        private GameObject chunkObject;

        public Chunk(Vector2Int chunkPosition, int size, World.GenerationSettings generationSettings)
        {
            this.size = size;
            this.chunkPosition = chunkPosition;
            worldPosition = chunkPosition * size;

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
            chunkObject = GameObject.Instantiate(prefab);
            chunkObject.name = $"Chunk ({chunkPosition.x}, {chunkPosition.y})";
            chunkObject.transform.position = worldPosition * 10.0f;

            MapDisplay mapDisplay = chunkObject.GetComponentInChildren<MapDisplay>();
            mapDisplay.DrawTexture(TextureGenerator.TextureFromHeightMap(noiseMap.DataPointArray2D));
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(chunkObject);
        }
    }
}