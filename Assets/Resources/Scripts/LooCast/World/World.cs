using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System;

namespace LooCast.World
{
    public class World : MonoBehaviour
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            //Perlin
            public int seed;
            public float scale;
            public float amplitude;
            public int octaves;
            public float persistence;
            public float lacunarity;
        }
        #endregion

        #region Voids
        private Vector2Int[] voids;
        #endregion

        #region Universe
        //How many Filament Chunks fit into the Universe (Per Axis)
        [SerializeField] private int universeChunkSize;
        #endregion

        #region Filament Chunks
        //How many Chunks fit into a Filament Chunk (Per Axis)
        [SerializeField] private int filamentChunkSize;
        [SerializeField] private Vector2Int filamentChunkAmount;
        #endregion

        #region Chunks
        //How big a Chunk is in Units (Per Axis)
        [SerializeField] private int chunkSize;
        [SerializeField] private GameObject chunkPrefab;
        [SerializeField] private GenerationSettings generationSettings;
        #endregion

        #region DEVELOPMENT
        [SerializeField] private Vector2Int[] chunkCoordinates;
        #endregion

        private Dictionary<Vector2Int, Chunk> loadedChunks = new Dictionary<Vector2Int, Chunk>();

        private void Start()
        {

        }

        public void DEV_GenerateChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                GenerateChunk(chunkCoordinate);
            }
        }

        public void DEV_LoadChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                LoadChunk(chunkCoordinate);
            }
        }

        public void DEV_UnloadChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                UnloadChunk(chunkCoordinate);
            }
        }

        public void DEV_SpawnChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                SpawnChunk(chunkCoordinate);
            }
        }

        public void DEV_DespawnChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                DespawnChunk(chunkCoordinate);
            }
        }

        #region Filament Chunks

        #endregion

        #region Chunks
        public Chunk GetChunk(Vector2Int chunkPosition)
        {
            if (!IsChunkLoaded(chunkPosition))
            {
                throw new Exception("Chunk is not loaded!");
            }

            if (!IsChunkGenerated(chunkPosition))
            {
                throw new Exception("Chunk is not generated!");
            }

            return loadedChunks[chunkPosition];
        }

        #region Chunk Generation
        private bool IsChunkGenerated(Vector2Int chunkPosition)
        {
            string path = $"{Application.dataPath}/Data/World/Chunks/{chunkPosition.x}.{chunkPosition.y}.json";
            return File.Exists(path);
        }

        private void GenerateChunk(Vector2Int chunkPosition)
        {
            if (IsChunkGenerated(chunkPosition))
            {
                throw new Exception("Chunk is already generated!");
            }
            Chunk chunk = new Chunk(chunkPosition, chunkSize, generationSettings);
            SaveChunk(chunk);
        }
        #endregion

        #region Chunk Saving
        private void SaveChunk(Chunk chunk)
        {
            string path = $"{Application.dataPath}/Data/World/Chunks/{chunk.ChunkPosition.x}.{chunk.ChunkPosition.y}.json";
            string json = JsonUtility.ToJson(chunk, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Chunk Loading
        private bool IsChunkLoaded(Vector2Int chunkPosition)
        {
            return loadedChunks.ContainsKey(chunkPosition);
        }

        private void LoadChunk(Vector2Int chunkPosition)
        {
            if (IsChunkLoaded(chunkPosition))
            {
                throw new Exception("Chunk is already loaded!");
            }

            if (!IsChunkGenerated(chunkPosition))
            {
                throw new Exception($"Chunk has not been generated yet!");
            }

            string path = $"{Application.dataPath}/Data/World/Chunks/{chunkPosition.x}.{chunkPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            loadedChunks.Add(chunkPosition, JsonUtility.FromJson<Chunk>(json));
        }

        private void UnloadChunk(Vector2Int chunkPosition)
        {
            if (!IsChunkLoaded(chunkPosition))
            {
                throw new Exception("Chunk is already unloaded!");
            }

            loadedChunks.Remove(chunkPosition);
        }
        #endregion

        #region Chunk Spawning
        private void SpawnChunk(Vector2Int chunkPosition)
        {
            GetChunk(chunkPosition).Spawn(chunkPrefab);
        }
        #endregion

        #region Chunk Despawning
        private void DespawnChunk(Vector2Int chunkPosition)
        {
            GetChunk(chunkPosition).Despawn();
        }
        #endregion
        #endregion
    }
}